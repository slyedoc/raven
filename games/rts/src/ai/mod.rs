mod health;
pub use health::*;

mod movement;
pub use movement::*;

mod attack;
pub use attack::*;

mod dead;
pub use dead::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((attack::plugin, dead::plugin, movement::plugin))
        .add_observer(on_goal)
        .add_systems(
            Update,
            (
                draw_path,
                //draw_attack_range.run_if(in_editor))
                draw_target,
            ),
        );
}

#[derive(Event, Copy, Clone, Debug)]
pub enum Goal {
    /// attack target, will move to target if not in range, stand still if in range and attack
    /// on cooldown,    
    Attack(Entity),
    /// Move to target position
    Move(Vec3),
    /// Move to target position and attack anything along the way
    AttackMove(Vec3),
}

pub fn on_goal(
    trigger: Trigger<Goal>,
    mut commands: Commands,
    query: Query<(&Transform, Has<Dead>)>,
) {
    let e = trigger.target();

    let Ok((_transform, has_dead)) = query.get(e) else {
        return;
    };

    commands.entity(e).remove::<(Path, Target, Aggro)>();

    if has_dead {
        return;
    }

    match trigger.event() {
        Goal::Move(target) => {
            commands.entity(e).insert((Aggro, Destination(*target)));
        }
        Goal::AttackMove(target) => {
            commands.entity(e).insert((Aggro, Destination(*target)));
        }
        Goal::Attack(enemy) => match query.get(*enemy) {
            Ok((enemy_transform, _)) => {
                commands
                    .entity(e)
                    .insert((Destination(enemy_transform.translation), Target(*enemy)));
            }
            Err(_) => {
                error!("Enemy not found");
            }
        },
    }
}

/// Characters that can revive
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Hero;

#[derive(Component)]
pub struct SensorRange(pub f32);

impl Default for SensorRange {
    fn default() -> Self {
        Self(15.0)
    }
}

#[allow(dead_code)]
pub(super) fn draw_attack_range(query: Query<(&Transform, &Attack)>, mut gizmos: Gizmos) {
    for (trans, attack) in query.iter() {
        gizmos.circle(
            Isometry3d::new(
                vec3(trans.translation.x, 0.01, trans.translation.z),
                Quat::from_rotation_arc(Vec3::Z, Vec3::Y),
            ),
            attack.range,
            tailwind::YELLOW_500,
        );
    }
}

pub(super) fn draw_target(
    query: Query<(&Transform, &Target)>,
    target_query: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for (trans, target) in query.iter() {
        if let Ok(target_gt) = target_query.get(target.0) {
            gizmos.line(trans.translation, target_gt.translation, tailwind::RED_500);
        }
    }
}
pub(super) fn draw_path(query: Query<(&Transform, &Path)>, mut gizmos: Gizmos) {
    for (trans, path) in query.iter() {
        let mut to_display = path.next.clone();
        to_display.push(path.current);
        to_display.push(trans.translation);
        to_display.reverse();
        if !to_display.is_empty() {
            gizmos.linestrip(
                to_display.iter().map(|xz| Vec3::new(xz.x, 0.1, xz.z)),
                tailwind::YELLOW_400,
            );
        }
    }
}
