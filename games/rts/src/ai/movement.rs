use bevy::ecs::error;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_destination)
        .add_systems(
            FixedUpdate,
            (
                update_path_info,
                (move_navigator, refresh_path),
                apply_movement_damping,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
}

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(pub f32);

/// The damping factor used for slowing down movement.
#[derive(Component, Deref, DerefMut)]
pub struct MovementDampingFactor(pub f32);

// The strength of a jump.
// #[derive(Component)]
// pub struct JumpImpulse(pub f32);

// The maximum angle a slope can have for a character controller
// to be able to climb and jump. If the slope is steeper than this angle,
// the character will slide down.
// #[derive(Component)]
// pub struct MaxSlopeAngle(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Speed {
    pub current: f32,
    pub max: f32,
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self {
            current: speed,
            max: speed,
        }
    }
}

/// Added to create a path is needed
#[derive(Component, Debug)]
pub struct Destination(pub Vec3);

fn update_destination(
    query: Query<(Entity, &Transform, &Destination)>,
    nav_query: Single<Entity, With<Nav>>,    
    mut commands: Commands,
    mut nav_path: NavPath,
) {    

    let nav_e = nav_query.into_inner();
    for (e, transform, destination) in query.iter() {
        let start = transform.translation;
        let end = destination.0;
        match nav_path.find_path(
            nav_e,
            start,
            end,
            None,              // no filter
            Some(&[1.0, 0.5]), // no filter
        ) {
            Ok(path) => match path.split_first() {
                Some((first, remaining)) => {
                    let mut remaining = remaining.to_vec();
                    remaining.reverse();
                    let path = Path {
                        current: *first,
                        next: remaining,
                        target: PathTarget::Position(end),
                    };
                    commands
                        .entity(e)
                        .insert((PathInfo::new(path.distance(transform.translation)), path))
                        .remove::<Destination>();
                }
                None => {
                    error!(" Path has only one point");
                }
            },
            Err(_) => error!("Error finding path from {:?} to {:?}", start, end),
        }
    }

    //     let start = find_closest(navmesh, from, to);
    //     let end = find_closest(navmesh, to, from);

    //     match navmesh.transformed_path(start, end) {
    //         Some(path) => match path.path.split_first() {
    //             Some((first, remaining)) => {
    //                 let mut remaining = remaining.to_vec();
    //                 remaining.reverse();
    //                 let path = Path {
    //                     current: *first,
    //                     next: remaining,
    //                     target: PathTarget::Position(end),
    //                 };
    //                 commands
    //                     .entity(e)
    //                     .insert((PathInfo::new(path.distance(transform.translation)), path))
    //                     .remove::<Destination>();
    //             }
    //             None => {
    //                 error!(" Path has only one point");
    //             }
    //         },
    //         None => {
    //             let dst = from.distance(to);
    //             let dst2 = start.distance(end);
    //             error!("No path found: {}, {}", dst, dst2);
    //         }
    //     }
    // }
}

// fn find_closest(navmesh: &mut NavMesh, from: Vec2, to: Vec2) -> Vec3 {
//     match navmesh
//         .get()
//         .get_closest_point_towards(from, to)
//         //.get_closest_point(from)
//         .map(|p| vec3(p.position().x, 0.0, p.position().y))
//     {
//         Some(start) => start, // found within navmesh
//         None => {
//             // we are off navmesh
//             let mut dst = f32::MAX;
//             let mut closest = Vec2::ZERO;
//             for l in navmesh.get().layers.iter() {
//                 for v in l.vertices.iter() {
//                     if from.distance(v.coords) < dst {
//                         dst = from.distance(v.coords);
//                         closest = v.coords;
//                     }
//                 }
//             }
//             vec3(closest.x, 0.0, closest.y)
//         }
//     }
// }

#[derive(Component)]
pub struct Path {
    /// Where we are going
    pub current: Vec3,
    /// Where we are going next
    pub next: Vec<Vec3>,
    /// Target of the path
    pub target: PathTarget,
}

impl Path {
    pub fn distance(&self, current_position: Vec3) -> f32 {
        let mut to_display = self.next.clone();
        to_display.push(self.current);
        to_display.push(current_position);
        to_display.reverse();
        // sum the distance between each point
        let mut total_distance = 0.0;
        for (a, b) in to_display.iter().zip(to_display.iter().skip(1)) {
            total_distance += a.distance(*b);
        }
        total_distance
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PathInfo {
    /// Where we started
    pub init_path_distance: f32,
    pub path_distance: f32,
}

impl PathInfo {
    pub fn new(init_path_distance: f32) -> Self {
        Self {
            init_path_distance,
            path_distance: 0.0,
        }
    }
}

impl Percentage for PathInfo {
    fn value(&self) -> f32 {
        self.path_distance / self.init_path_distance
    }
}

pub fn update_path_info(mut query: Query<(&Transform, &Path, &mut PathInfo)>) {
    for (trans, path, mut path_info) in query.iter_mut() {
        path_info.path_distance = path.distance(trans.translation);
    }
}

pub enum PathTarget {
    #[allow(dead_code)]
    Entity(Entity),
    Position(Vec3),
}

pub(super) fn move_navigator(
    mut commands: Commands,
    mut navigator: Query<(
        Entity,
        &mut ExternalImpulse,
        //&mut LinearVelocity,
        &MovementAcceleration,
        &Transform,
        &mut Path,
        &mut Speed,
        Option<&Attack>,
        Option<&Target>,
    )>,
    time: Res<Time<Physics>>,
    target_query: Query<&Transform>,
) {
    const DIST_TO_NEXT: f32 = 0.1;
    for (
        e,
        mut external_impluse,
        //linear_vel,
        movment_acc,
        transform,
        mut path,
        speed,
        attack_range,
        target,
    ) in navigator.iter_mut()
    {
        // if we have a target, dont move closer than our attack range
        if let Some(target) = target {
            let Ok(target_trans) = target_query.get(target.0) else {
                commands.entity(e).remove::<Target>();
                dbg!("Target missing");
                continue;
            };
            if let Some(ar) = attack_range {
                let target_disnce = transform.translation.distance(target_trans.translation);
                if target_disnce < ar.range {
                    commands.entity(e).remove::<Path>();
                    continue;
                }
            } else {
                dbg!("No attack range");
            }
        }

        let move_direction = path.current.xz() - transform.translation.xz();
        let movement = move_direction.normalize() * time.delta_secs() * speed.current;
        external_impluse.set_impulse(vec3(
            movement.x * movment_acc.0,
            0.0,
            movement.y * movment_acc.0,
        ));

        let mut distance_to_next = transform.translation.xz().distance(path.current.xz());
        while distance_to_next < DIST_TO_NEXT {
            if let Some(next) = path.next.pop() {
                path.current = next;
                distance_to_next = transform.translation.distance(path.current);
            } else {
                commands.entity(e).remove::<Path>();
                //dbg!("Path done");
                break;
            }
        }
    }
}

/// Slows down movement in the XZ plane.
pub(super) fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>,
) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        if linear_velocity.x.abs() < 0.001 {
            linear_velocity.x = 0.0;
        }
        linear_velocity.z *= damping_factor.0;
        if linear_velocity.z.abs() < 0.001 {
            linear_velocity.z = 0.0;
        }
    }
}

pub(super) fn refresh_path(
    mut commands: Commands,
    mut character: Query<(Entity, &Transform, &mut Path)>,
    //mut navmeshes: ResMut<Assets<NavMesh>>,
    //navmesh: Query<(&NavMeshWrapper, Ref<NavMeshStatus>)>,
    transforms: Query<&Transform>,
    mut deltas: Local<EntityHashMap<f32>>,
) {
    // let (navmesh_handle, status) = navmesh.single();
    // if (!status.is_changed() || *status != NavMeshStatus::Built) && deltas.is_empty() {
    //     return;
    // }
    // let Some(navmesh) = navmeshes.get_mut(&navmesh_handle.0) else {
    //     return;
    // };

    // for (entity, transform, mut path) in &mut character {
    //     let target = match path.target {
    //         PathTarget::Entity(entity) => {
    //             let Ok(t) = transforms.get(entity) else {
    //                 dbg!("No transform");
    //                 continue;
    //             };
    //             t.translation
    //         }
    //         PathTarget::Position(vec3) => vec3,
    //     };
    //     navmesh.set_search_delta(0.0);
    //     if !navmesh.transformed_is_in_mesh(transform.translation) {
    //         let delta_for_entity = deltas.entry(entity).or_insert(0.0);
    //         *delta_for_entity += 0.1;
    //         navmesh.set_search_delta(*delta_for_entity);
    //         continue;
    //     }
    //     if !navmesh.transformed_is_in_mesh(target) {
    //         commands.entity(entity).remove::<Path>();
    //         continue;
    //     }

    //     let Some(new_path) = navmesh.transformed_path(transform.translation, target) else {
    //         commands.entity(entity).remove::<Path>();
    //         continue;
    //     };
    //     if let Some((first, remaining)) = new_path.path.split_first() {
    //         let mut remaining = remaining.to_vec();
    //         remaining.reverse();
    //         path.current = *first;
    //         path.next = remaining;
    //         deltas.remove(&entity);
    //     }
    // }
}
