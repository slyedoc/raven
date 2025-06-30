use std::{any::TypeId, cmp::Ordering, collections::HashMap};

use bevy::{
    app::PluginGroupBuilder,
    ecs::{
        component::ComponentId,
        intern::Interned,
        schedule::ScheduleLabel,
        system::{IntoObserverSystem, SystemId},
    },
    prelude::*,
};

pub mod prelude {
    pub use super::{
        Action, ActionFinished, GalaxyBrainPlugins, GalaxyBrainSet,
        ObserveWithComponentLifetime as _, RegisterAction as _, Scorer,
    };
}

pub struct GalaxyBrainPlugins {
    schedule: Interned<dyn ScheduleLabel>,
}

impl GalaxyBrainPlugins {
    pub fn new(schedule: impl ScheduleLabel) -> Self {
        Self {
            schedule: schedule.intern(),
        }
    }
}

impl PluginGroup for GalaxyBrainPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GalaxyBrainCorePlugin::new(self.schedule))
            .add(HighestScorePickerPlugin)
    }
}

pub struct GalaxyBrainCorePlugin {
    schedule: Interned<dyn ScheduleLabel>,
}

impl GalaxyBrainCorePlugin {
    pub fn new(schedule: impl ScheduleLabel) -> Self {
        Self {
            schedule: schedule.intern(),
        }
    }
}

#[derive(Resource, Debug)]
pub struct GalaxyBrainConfig {
    schedule: Interned<dyn ScheduleLabel>,
    action_creators: HashMap<ComponentId, SystemId<Entity, ()>>,
    pub registered_actions_info: Vec<RegisteredActionInfo>,
}

#[derive(Debug)]
pub struct RegisteredActionInfo {
    pub action_id: ComponentId,
    pub action_name: String,
    pub scorer_id: ComponentId,
    pub scorer_type_id: TypeId,
}

impl Plugin for GalaxyBrainCorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Scores>();
        app.register_type::<CurrentAction>();

        app.configure_sets(
            self.schedule.intern(),
            (
                (
                    GalaxyBrainSet::UserScorers,
                    GalaxyBrainSet::ClearPreviousScores,
                ),
                GalaxyBrainSet::ScoreCollection,
                GalaxyBrainSet::ActionPicking,
                GalaxyBrainSet::TransitionActions,
                GalaxyBrainSet::UserActions,
            )
                .chain(),
        );

        app.insert_resource(GalaxyBrainConfig {
            schedule: self.schedule.intern(),
            action_creators: HashMap::new(),
            registered_actions_info: Vec::new(),
        });

        app.add_systems(
            self.schedule.intern(),
            (
                clear_scores.in_set(GalaxyBrainSet::ClearPreviousScores),
                transition_actions.in_set(GalaxyBrainSet::TransitionActions),
            ),
        );

        app.world_mut().spawn((
            Name::new("GalaxyBrain::HandleActionFinished Observer"),
            Observer::new(handle_action_finished),
        ));
    }
}

pub struct HighestScorePickerPlugin;

impl Plugin for HighestScorePickerPlugin {
    fn build(&self, app: &mut App) {
        let schedule = app.world().resource::<GalaxyBrainConfig>().schedule;

        app.add_systems(
            schedule,
            action_picker_highest_score.in_set(GalaxyBrainSet::ActionPicking),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GalaxyBrainSet {
    UserScorers,
    ClearPreviousScores,
    ScoreCollection,
    ActionPicking,
    TransitionActions,
    UserActions,
}

pub trait Action: Sized {
    type Scorer: Component + Scorer;
}

pub trait Scorer {
    fn score(&self) -> f32;
}

pub trait RegisterAction {
    fn register_action<T: Component + Action, M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + 'static,
    ) -> &mut Self;
}

impl RegisterAction for App {
    fn register_action<T: Component + Action, M>(
        &mut self,
        init_action: impl IntoSystem<Entity, (), M> + 'static,
    ) -> &mut Self {
        let world = self.world_mut();

        let sys_id = world.register_system(init_action);

        let action_id = world.init_component::<T>();
        let action_name = world.components().get_name(action_id).unwrap().to_owned();
        let scorer_id = world.init_component::<T::Scorer>();

        let mut config = world.resource_mut::<GalaxyBrainConfig>();
        config.action_creators.insert(action_id, sys_id);
        config.registered_actions_info.push(RegisteredActionInfo {
            action_id,
            action_name,
            scorer_id,
            scorer_type_id: TypeId::of::<T::Scorer>(),
        });

        let schedule = config.schedule;

        self.add_systems(
            schedule,
            make_collect_score::<T::Scorer>(action_id).in_set(GalaxyBrainSet::ScoreCollection),
        );

        self
    }
}

#[derive(Bundle, Default)]
pub struct AiBundle<Picker: Component + Default> {
    pub scores: Scores,
    pub current_action: CurrentAction,
    pub picker: Picker,
}

/// Holds all the scores from [`Scorer`]s.
#[derive(Component, Reflect, Default, Debug)]
pub struct Scores {
    pub scores: Vec<(ComponentId, f32)>,
}

#[derive(Component, Reflect, Default, Debug)]
pub struct CurrentAction {
    pub component: Option<ComponentId>,
    pub last_action: Option<ComponentId>,
}

#[derive(Component, Default, Debug)]
pub struct ActionPickerHighestScore;

fn action_picker_highest_score(mut query: Query<&mut Scores, With<ActionPickerHighestScore>>) {
    query.iter_mut().for_each(|mut scores| {
        scores.scores.sort_unstable_by(|(_, a), (_, b)| {
            if *a > *b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    });
}

fn make_collect_score<C: Component + Scorer>(
    component_id: ComponentId,
) -> impl Fn(Query<'_, '_, (&mut Scores, &C)>) {
    move |mut query: Query<(&mut Scores, &C)>| {
        query.iter_mut().for_each(|(mut scores, scorer)| {
            scores.scores.push((component_id, scorer.score()));
        });
    }
}

fn clear_scores(mut query: Query<&mut Scores>) {
    query.iter_mut().for_each(|mut scores| {
        scores.scores.clear();
    });
}

fn transition_actions(
    mut query: Query<(&mut CurrentAction, &Scores, Entity)>,
    mut cmd: Commands,
    config: Res<GalaxyBrainConfig>,
) {
    query
        .iter_mut()
        .for_each(|(mut current_action, scores, entity)| {
            let last_action = current_action.component;

            macro_rules! remove_current_action {
                ($id: ident) => {
                    cmd.entity(entity).remove_by_id($id);
                };
            }

            macro_rules! init_and_set_new_action {
                ($id: ident) => {
                    let action_creator_sys = config
                        .action_creators
                        .get($id)
                        .expect("expected action creator to have been registered");
                    cmd.run_system_with_input(*action_creator_sys, entity);
                    current_action.component = Some(*$id);
                    if last_action.is_none() || last_action.is_some_and(|l| l != *$id) {
                        current_action.last_action = last_action;
                    }
                };
            }

            match (scores.scores.first(), current_action.component) {
                (Some((new_action_id, _)), Some(current_action_id)) => {
                    if *new_action_id != current_action_id {
                        remove_current_action!(current_action_id);
                        init_and_set_new_action!(new_action_id);
                    }
                }
                (Some((new_action_id, _)), None) => {
                    init_and_set_new_action!(new_action_id);
                }
                (None, Some(current_action_id)) => {
                    remove_current_action!(current_action_id);
                }
                (None, None) => {}
            };
        });
}

#[derive(Event)]
pub struct ActionFinished;

fn handle_action_finished(
    trigger: Trigger<ActionFinished>,
    mut query: Query<&mut CurrentAction>,
    mut cmd: Commands,
) {
    if let Ok(mut current_action) = query.get_mut(trigger.entity()) {
        if let Some(current_action_id) = current_action.component {
            cmd.entity(trigger.entity()).remove_by_id(current_action_id);
            current_action.component = None;
        }
    }
}

pub trait ObserveWithComponentLifetime {
    fn observe_with_component_lifetime<C: Component, E: Event, B: Bundle, M>(
        &mut self,
        entity: Entity,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}

// TODO: make a version of this so multiple observers can be created that share a single cleanup observer
impl ObserveWithComponentLifetime for Commands<'_, '_> {
    fn observe_with_component_lifetime<C: Component, E: Event, B: Bundle, M>(
        &mut self,
        entity: Entity,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        // User's observer observing supplied entity
        let mut user_observer = Observer::new(system);
        user_observer.watch_entity(entity);
        let user_observer = self
            .spawn((Name::new("GalaxyBrain::User Observer"), user_observer))
            .id();

        // Observer that removes user's observer and itself upon component removal
        let mut cleanup_observer_cmd = self.spawn_empty();
        let cleanup_observer_id = cleanup_observer_cmd.id();
        let mut cleanup_obsever =
            Observer::new(move |_: Trigger<OnRemove, C>, mut cmd: Commands| {
                cmd.entity(user_observer).despawn();
                cmd.entity(cleanup_observer_id).despawn();
            });
        cleanup_obsever.watch_entity(entity);
        cleanup_observer_cmd.insert((Name::new("GalaxyBrain::Cleanup Observer"), cleanup_obsever));

        self
    }
}