use crate::{prelude::*, states::in_game_over::{InGameOver, Winner}};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
pub enum InScoreboard {
    #[default]
    Disabled,
    Active,
}

pub fn plugin(app: &mut App) {
    app.add_sub_state::<InScoreboard>()
        .enable_state_scoped_entities::<InScoreboard>()
        .add_systems(OnEnter(InScoreboard::Active), setup)
        .add_systems(
            Update,
            exit.run_if(input_just_pressed(KeyCode::Escape).and(in_state(InScoreboard::Active))),
        );
}

fn setup(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(6.0)),
                //row_gap: MARGIN,
                ..default()
            },
            StateScoped(InScoreboard::Active),
        ))
        .with_children(|b| {
            // b.spawn((
            //     Node {
            //         width: Val::Percent(50.),
            //         flex_direction: FlexDirection::Column,
            //         align_items: AlignItems::Center,
            //         ..default()
            //     },
            //     BackgroundColor(UI_BG),
            // ))
            // .with_children(|b| {
            b.spawn((
                Text::new("Scoreboard"),
                TextFont {
                    font: ui.font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(tailwind::GRAY_100.into()),
            ));

            // quit
            b.spawn(create_button_outer())
                .with_children(|b| {
                    b.spawn(create_button_text("Quit".into(), ui.font.clone()));
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>,
                     mut game_over: ResMut<NextState<InGameOver>>,
                     mut cmds: Commands,
                     player: Single<&Team, With<ActivePlayer>>| {
                        let player_team = *player;
                        cmds.insert_resource(Winner {
                            team: player_team.other(),
                        });

                        game_over.set(InGameOver::Active);
                    },
                );
            // });
        });
}

fn exit(mut next: ResMut<NextState<InScoreboard>>) {
    next.set(InScoreboard::Disabled);
}
