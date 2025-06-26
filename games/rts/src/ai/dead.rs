use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_dead)
        .add_observer(on_add_dead)
        .add_observer(on_remove_dead);
}

fn update_dead(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Dead)>,
    time: Res<Time<Physics>>,
) {
    for (e, mut dead) in query.iter_mut() {
        if dead.0.tick(time.delta()).just_finished() {
            commands.entity(e).remove::<Dead>();
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Dead(pub Timer);

fn on_add_dead(
    trigger: Trigger<OnAdd, Dead>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Visibility, &Team), With<Dead>>,
    camera: Single<Entity, With<GameCamera>>,
    player: Single<Entity, With<ActivePlayer>>,
) {
    let e = trigger.target();
    let player = *player;
    let camera = *camera;
    // remove other conditions
    commands
        .entity(e)
        //.remove::<Engage>()
        .remove::<Path>();

    let Ok((mut transform, mut vis, team)) = query.get_mut(e) else {
        error!("Missing componet for dead: {}", e);
        return;
    };

    transform.translation = team.respawn_point();

    // hide the entity
    *vis = Visibility::Hidden;

    // setup death shader
    if e == player {
        commands.entity(camera).insert(DeadPostProcessSettings {
            intensity: 0.02,
            ..default()
        });
    }
}

fn on_remove_dead(
    trigger: Trigger<OnRemove, Dead>,
    mut commands: Commands,
    mut query: Query<(&mut Health, &mut Visibility)>,
    player: Single<Entity, With<ActivePlayer>>,
    camera: Single<Entity, With<GameCamera>>,
) {
    let e = trigger.target();
    let player = *player;
    let camera = *camera;

    let Ok((mut health, mut vis)) = query.get_mut(e) else {
        error!("Missing componet for dead: {}", e);
        return;
    };

    health.reset();
    *vis = Visibility::Visible;

    if e == player {
        commands.entity(camera).remove::<DeadPostProcessSettings>();
    }
}
