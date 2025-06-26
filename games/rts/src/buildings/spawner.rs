use crate::{prelude::*, states::in_paused::InPaused};

pub(super) fn plugin<T: Component>(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_spawners::<T>.run_if(in_state(InPaused::Disabled)),),
    )
    .add_systems(Update, draw_debug::<T>);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Team)]

pub struct Spawner<T: Component> {
    pub timer: Timer,
    pub marker: PhantomData<T>,
}

impl<T: Component> Default for Spawner<T> {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(10.0, TimerMode::Repeating),
            marker: PhantomData::<T>,
        }
    }
}

fn update_spawners<T: Component>(
    time: Res<Time<Physics>>,
    mut query: Query<(&GlobalTransform, &mut Spawner<T>, &Team)>,
    mut commands: Commands,
) {
    for (gt, mut spawner, team) in query.iter_mut() {
        if spawner.timer.tick(time.delta()).just_finished() {
            for i in 0..3 {
                commands
                    .spawn((
                        Transform::from_translation(gt.translation() + vec3(i as f32 * 2., 0., 0.)),
                        Footmen,
                        *team,
                    ))
                    .trigger(team.goal());
            }
        }
    }
}

fn draw_debug<T: Component>(
    query: Query<(&GlobalTransform, &Team), With<Spawner<T>>>,
    mut gizmos: Gizmos,
) {
    for (trans, team) in query.iter() {
        gizmos.sphere(trans.translation(), 1.0, team.color());
    }
}
