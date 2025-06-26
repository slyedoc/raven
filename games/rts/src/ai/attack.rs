use std::cmp::Ordering;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_add_aggo)
        .add_observer(on_remove_aggo)
        .add_systems(
            Update,
            (
                (attack_cd, attack_if_ready).chain(),
                check_target_valid,
                update_target_if_aggro::<Footmen>,
                update_target_if_aggro::<Tower>,
            ),
        );
}

/// Will be automatic added when [`Aggro`] enemy is in [`SensorRange`], removed if out of range or dead
#[derive(Component, Debug)]
pub struct Target(pub Entity);

/// While this is present, the entity will auto target anything
#[derive(Component, Default, Debug)]
#[require(SensorRange)]
#[require(Team)]
pub struct Aggro;

#[derive(Component, Debug)]

pub struct AggroSensor;

#[derive(Component, Debug, Reflect, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Attack {
    pub dmg: f32,
    pub range: f32,
    pub timer: Timer,
}

impl Attack {
    pub fn new(dmg: f32, range: f32, duration: f32) -> Self {
        let mut timer = Timer::from_seconds(duration, TimerMode::Once);
        timer.pause();
        Self { dmg, range, timer }
    }
}

impl Percentage for Attack {
    fn value(&self) -> f32 {
        self.timer.fraction()
    }
}

pub(super) fn attack_cd(mut query: Query<&mut Attack>, time: Res<Time<Physics>>) {
    for mut attack in query.iter_mut() {
        if attack.timer.tick(time.delta()).just_finished() {
            attack.timer.reset();
            attack.timer.pause();
        }
    }
}

pub(super) fn attack_if_ready(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Attack, &Target), Without<Path>>,
) {
    for (e, transform, mut attack, target) in query.iter_mut() {
        if attack.timer.paused() {
            attack.timer.unpause();

            // fire
            commands.spawn((
                Projectile {
                    dmg: attack.dmg,
                    target: target.0,
                    from: e,
                },
                Transform::from_translation(transform.translation),
            ));
        }
    }
}

pub(super) fn on_add_aggo(
    trigger: Trigger<OnAdd, Aggro>,
    mut commands: Commands,
    query: Query<(&Team, &SensorRange)>,
) {
    let e = trigger.target();
    let (team, sr) = query.get(e).unwrap();
    commands.entity(e).with_children(|b| {
        b.spawn((
            Sensor,
            team.sensors(),
            CollidingEntities::default(),
            Collider::sphere(sr.0),
            Pickable::IGNORE,
            AggroSensor,
        ));
    });
}

pub(super) fn on_remove_aggo(
    trigger: Trigger<OnAdd, Aggro>,
    mut commands: Commands,
    query: Query<&Children>,
    aggro_sensor_query: Query<&AggroSensor>,
) {
    let e = trigger.target();
    if let Ok(children) = query.get(e) {
        for child in children.iter() {
            if aggro_sensor_query.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }
}

pub(super) fn check_target_valid(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &SensorRange, &Target)>,
    target_query: Query<&Transform>,
    mut ray_cast: MeshRayCast,
) {
    const SENSOR_FUDGE: f32 = 0.1; // fudge factor to make sure we don't remove target too early
    for (e, trans, sensor_range, target) in query.iter_mut() {
        if let Ok(target_trans) = target_query.get(target.0) {
            if let Some((_, hit)) = ray_cast
                .cast_ray(
                    Ray3d::new(
                        trans.translation,
                        Dir3::new(
                            (target_trans.translation - trans.translation).normalize_or_zero(),
                        )
                        .unwrap(),
                    ),
                    &MeshRayCastSettings {
                        early_exit_test: &|e| e != target.0,
                        ..default()
                    },
                )
                .first()
            {
                if hit.distance > sensor_range.0 + SENSOR_FUDGE {
                    commands.entity(e).remove::<Target>();
                }
            } else {
                dbg!("Raycast failed");
            }
        } else {
            commands.entity(e).remove::<Target>();
        }
    }
}

pub(super) fn update_target_if_aggro<T: Component>(
    mut commands: Commands,
    mut sensor_query: Query<(&ChildOf, &CollidingEntities), With<Sensor>>,
    query: Query<(Entity, &Transform), (With<T>, Without<Target>, With<Aggro>)>,
    // targetings
    target_query: Query<&Transform>,
) {
    for (parent, CollidingEntities(entities)) in sensor_query.iter_mut() {
        if let Ok((e, trans)) = query.get(parent.parent()) {
            // find nearest target
            let mut distances = entities
                .iter()
                .filter_map(|e| {
                    // target exists
                    if let Ok(target_gt) = target_query.get(*e) {
                        return Some((*e, trans.translation.distance(target_gt.translation)));
                    }
                    None
                })
                .collect::<Vec<_>>();
            distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or_else(|| Ordering::Equal));

            // if we have a target, chase it
            if let Some((target_e, _)) = distances.iter().next() {
                commands.entity(e).trigger(Goal::Attack(*target_e));
            }
        }
    }
}
