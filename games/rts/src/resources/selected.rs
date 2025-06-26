use bevy::platform::collections::HashMap;

use crate::prelude::*;

const SELECT_GROUP_KEYS: [KeyCode; 6] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
];

pub fn plugin(app: &mut App) {
    app.init_resource::<Selected>()
        .init_resource::<SelectedGroups>()
        .init_resource::<CursorPos>()
        .add_systems(Update, (valid_selected, selection_groups, draw_selected))
        .add_systems(
            Update,
            (
                get_cursor_world_pos,
                //(
                // start_drag.run_if(input_just_pressed(MouseButton::Left)),
                // end_drag.run_if(input_just_released(MouseButton::Left)),
                // drag.run_if(resource_exists::<CursorDrag>),
                //),
            ), //.chain(),
        )
        .add_systems(
            OnExit(AppState::InGame),
            |mut selected: ResMut<Selected>, mut selected_groups: ResMut<SelectedGroups>| {
                selected.clear();
                selected_groups.clear();
            },
        );
}
fn valid_selected(mut selected: ResMut<Selected>, query: Query<&Transform>) {
    selected.retain(|e| query.get(*e).is_ok());
}

fn selection_groups(
    inputs: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    mut selected_groups: ResMut<SelectedGroups>,
) {
    let mut key = None;
    for k in SELECT_GROUP_KEYS.iter() {
        if inputs.just_pressed(*k) {
            key = Some(k);
            break;
        }
    }
    let Some(key) = key else {
        return;
    };

    if inputs.pressed(KeyCode::ControlLeft) {
        selected_groups.insert(*key, selected.iter().copied().collect());
    } else {
        // Select group
        if !inputs.pressed(KeyCode::ShiftLeft) {
            selected.clear();
        }
        if let Some(group) = selected_groups.get(key) {
            selected.extend(group.iter().copied());
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Selected(pub Vec<Entity>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SelectedGroups(pub HashMap<KeyCode, Vec<Entity>>);

fn draw_selected(selected: Res<Selected>, query: Query<&Transform>, mut gizmos: Gizmos) {
    for e in selected.iter() {
        if let Ok(transform) = query.get(*e) {
            gizmos.circle(
                Isometry3d::new(
                    vec3(transform.translation.x, 0.01, transform.translation.z),
                    Quat::from_rotation_arc(Vec3::Z, Vec3::Y),
                ),
                1.0,
                tailwind::GREEN_300,
            );
        }
    }
}

/// The projected 2D world coordinates of the cursor (if it's within primary window bounds).
#[derive(Resource, Default)]
struct CursorPos {
    world: Option<Vec3>,
}

/// The current drag operation including the offset with which we grabbed the Bevy logo.
#[derive(Resource)]
struct CursorDrag {
    world: Vec3,
}

/// Project the cursor into the world coordinates and store it in a resource for easy use
fn get_cursor_world_pos(
    mut cursor_pos: ResMut<CursorPos>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (main_camera, main_camera_transform) = *q_camera;

    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };

    // cursor_pos.screen = Some(cursor_position);

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = main_camera.viewport_to_world(main_camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    cursor_pos.world = Some(ray.get_point(distance));
}
/// Start the drag operation and record the offset we started dragging from
#[allow(dead_code)]
fn start_drag(mut commands: Commands, cursor_pos: Res<CursorPos>) {
    // If the cursor is not within the primary window skip this system
    let Some(world) = cursor_pos.world else {
        return;
    };
    // let Some(screen) = cursor_pos.screen else {
    //     return;
    // };
    commands.insert_resource(CursorDrag { world });
}

/// Stop the current drag operation
#[allow(dead_code)]
fn end_drag(mut commands: Commands) {
    commands.remove_resource::<CursorDrag>();
}

/// Drag the Bevy logo
#[allow(dead_code)]
fn drag(
    cursor_drag: Res<CursorDrag>,
    cursor_pos: Res<CursorPos>,
    canidates: Query<(Entity, &Team, &Transform), With<Unit>>,
    mut selected: ResMut<Selected>,
    inputs: Res<ButtonInput<KeyCode>>,
    player: Single<&Team, With<ActivePlayer>>,
    mut gizmos: Gizmos,
) {
    let player_team = *player;
    // If the cursor is not within the primary window skip this system
    let Some(world_pos) = cursor_pos.world else {
        return;
    };

    gizmos.circle(
        Isometry3d::new(
            vec3(world_pos.x, 0.01, world_pos.z),
            Quat::from_rotation_arc(Vec3::Z, Vec3::Y),
        ),
        0.2,
        Color::WHITE,
    );
    gizmos.line(world_pos, cursor_drag.world, Color::WHITE);

    let mid_point = (world_pos + cursor_drag.world) / 2.0;
    let size = Vec2::new(
        (world_pos.x - cursor_drag.world.x).abs(),
        (world_pos.z - cursor_drag.world.z).abs(),
    );
    gizmos.rect(
        Isometry3d::new(mid_point, Quat::from_rotation_arc(Vec3::Z, Vec3::Y)),
        size,
        Color::WHITE,
    );

    // update selected
    if !inputs.pressed(KeyCode::ShiftLeft) {
        selected.clear();
    }

    canidates
        .iter()
        .filter(|(_, team, _)| *team == player_team)
        .filter_map(|(e, _, transform)| {
            let x = transform.translation.x;
            let z = transform.translation.z;
            if x > cursor_drag.world.x.min(world_pos.x)
                && x < cursor_drag.world.x.max(world_pos.x)
                && z > cursor_drag.world.z.min(world_pos.z)
                && z < cursor_drag.world.z.max(world_pos.z)
            {
                Some(e)
            } else {
                None
            }
        })
        .for_each(|e| {
            if !selected.contains(&e) {
                selected.push(e);
            }
        });
}
