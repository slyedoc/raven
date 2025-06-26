use core::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
    state::state::FreelyMutableState,
};
use bevy_tweening::{lens::*, *};

pub struct FadePlugin<T: FreelyMutableState> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FreelyMutableState> Default for FadePlugin<T> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: FreelyMutableState> Plugin for FadePlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }

        app.add_event::<FadeTo<T>>()
            .add_systems(Update, (on_fade_to::<T>, loop_complete::<T>));
    }
}

#[derive(Event, Debug, Deref, DerefMut)]
pub struct FadeTo<T>(pub T)
where
    T: FreelyMutableState;

// Component for the fade overlay, with
#[derive(Component)]
#[require(
    GlobalZIndex = GlobalZIndex(100),
    Node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        ..default()
    },
    BackgroundColor = BackgroundColor(Color::NONE),
)]
#[component(on_add = on_add_fade)]
struct FadeOverlay<T: FreelyMutableState> {
    // clear when used
    target: Option<T>,
}

fn on_add_fade(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs_f32(0.2),
        UiBackgroundColorLens {
            start: Color::NONE,
            end: Color::BLACK,
        },
    )
    .with_repeat_count(RepeatCount::Finite(2))
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
    // Get an event after each segment
    .with_completed_event(0);

    world.commands().entity(entity).insert(Animator::new(tween));
}

// converts event to entity
pub fn on_fade_to<T: FreelyMutableState>(mut commands: Commands, mut added: EventReader<FadeTo<T>>) {
    for added in added.read() {
        commands.spawn(FadeOverlay {
            target: Some(added.0.clone()),
        });
    }
}

fn loop_complete<T: FreelyMutableState>(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,

    mut fade: Query<&mut FadeOverlay<T>>,
) {
    for ev in reader.read() {
        if let Ok(mut fade) = fade.get_mut(ev.entity) {
            if let Some(target) = &mut fade.target.take() {
                // navigate to the next state, first time, fade will reverse
                commands.set_state(target.clone());
            } else {
                // despawn the overlay, second time
                commands.entity(ev.entity).despawn();
            }
        }
    }
}