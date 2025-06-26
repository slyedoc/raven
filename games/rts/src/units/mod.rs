use crate::prelude::*;

mod merchant;
mod worker;
pub use worker::*;

mod footmen;
pub use footmen::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        //miner::plugin,
        merchant::plugin,
        worker::plugin,
        footmen::plugin,
    ))
    .add_observer(on_add_unit);
}

#[derive(Component, Default)]
#[require(Transform)]
#[require(Team)]
#[require(RigidBody = RigidBody::Dynamic)]
#[require(LockedAxes = LockedAxes::ROTATION_LOCKED)]
#[require(NavMeshAffector)]
#[require(StateScoped::<AppState> = StateScoped(AppState::InGame))]
pub struct Unit;

fn on_add_unit(
    trigger: Trigger<OnAdd, Unit>,
    mut commands: Commands,
    query: Query<&Team>,
    team_ass: Res<TeamAssets>,
) {
    let e = trigger.target();
    let team = query.get(e).unwrap();
    commands
        .entity(e)
        .observe(on_damage)
        .observe(color_on::<Pointer<Over>>(SELECT_MATERIAL_HOVER))
        .observe(color_on::<Pointer<Out>>(team.material(&team_ass)))
        .observe(color_on::<Pointer<Pressed>>(SELECT_MATERIAL_PRESSED))
        .observe(color_on::<Pointer<Released>>(SELECT_MATERIAL_HOVER))
        .observe(select);
}

fn select(
    trigger: Trigger<Pointer<Click>>,
    mut selected: ResMut<Selected>,
    player: Single<&Team, With<ActivePlayer>>,
    query: Query<&Team, Without<ActivePlayer>>,
    mut commands: Commands,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let e = trigger.target();
    let player_team = *player;
    let team = query.get(e).unwrap();

    // select if same team
    match team == player_team {
        true => {
            if !inputs.pressed(KeyCode::ShiftLeft) {
                selected.clear();
            }
            if !selected.contains(&e) {
                selected.push(e);
            }
        }
        false => {
            for e in selected.iter() {
                commands.entity(*e).trigger(Goal::Attack(trigger.target()));
            }
        }
    }
}
