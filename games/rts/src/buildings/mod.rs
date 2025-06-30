mod base;
pub use base::*;

mod smelter;
#[allow(unused_imports)]
pub use smelter::*;

mod spawner;
pub use spawner::*;

mod tower;
pub use tower::*;

use crate::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
#[require(Team)]
#[require(StateScoped::<AppState> = StateScoped(AppState::InGame) )]
pub struct Building;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        base::plugin,
        smelter::plugin,
        spawner::plugin::<Worker>,
        spawner::plugin::<Footmen>,
        tower::plugin,
    ))
    .add_observer(on_add_building);
}

fn on_add_building(
    trigger: Trigger<OnAdd, Building>,
    mut commands: Commands,
    query: Query<&Team>,
    team_ass: Res<TeamAssets>,
) {
    let e = trigger.target();
    let team = query.get(e).unwrap();

    commands
        .entity(e)
        .insert(team.collision_layers())
        .observe(on_damage)
        .observe(color_on::<Pointer<Over>>(SELECT_MATERIAL_HOVER))
        .observe(color_on::<Pointer<Out>>(team.material(&team_ass)))
        .observe(color_on::<Pointer<Pressed>>(SELECT_MATERIAL_PRESSED))
        .observe(color_on::<Pointer<Released>>(SELECT_MATERIAL_HOVER));
}
