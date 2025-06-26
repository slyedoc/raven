use crate::{prelude::*, states::in_paused::InPaused};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
pub enum InGameOver {
    #[default]
    Disabled,
    Active,
}

#[derive(Resource)]
pub struct Winner {
    pub team: Team,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_sub_state::<InGameOver>()
        .enable_state_scoped_entities::<InGameOver>()
        .add_systems(OnEnter(InGameOver::Active), setup)
        .add_systems(
            Update,
            exit.run_if(input_just_pressed(KeyCode::Escape).and(in_state(InGameOver::Active))),
        );
}

fn setup(
    mut commands: Commands,
    ui: Res<UiAssets>,

    winner: Res<Winner>,
    player: Single<&Team, With<ActivePlayer>>,
    mut time: ResMut<Time<Physics>>,
    mut paused: ResMut<NextState<InPaused>>,
) {
    time.pause();
    paused.set(InPaused::Disabled);

    let player_team = *player;

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
            StateScoped(InGameOver::Active),
        ))
        .with_children(|b| {
            b.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(6.0)),
                    //row_gap: MARGIN,
                    ..default()
                },
                BackgroundColor(UI_BG),
            ))
            .with_children(|b| {
                b.spawn((
                    Text::new("Victory"),
                    TextFont {
                        font: ui.font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(tailwind::GRAY_100.into()),
                ));

                // b.spawn(create_button_outer())
                //     .with_children(|b| {
                //         b.spawn(create_button_text("New Game".into(), ui.font.clone()));
                //     })
                //     .observe(
                //         |_trigger: Trigger<Pointer<Down>>,
                //          mut next: ResMut<NextState<AppState>>| {
                //             next.set(AppState::InGame);
                //         },
                //     );

                // b.spawn((
                //     Node::default(),
                //     Text::new("Here"),
                //     TextFont {
                //         font: ui.font.clone(),
                //         font_size: 20.0,
                //         ..default()
                //     },
                //     TextColor(LABEL_COLOR),
                // ));

                b.spawn((create_title(
                    match winner.team == *player_team {
                        true => "Victory",
                        false => "Defeat",
                    },
                    ui.font.clone(),
                ),));

                // Exit
                b.spawn(create_button_outer())
                    .with_children(|b| {
                        b.spawn(create_button_text("Exit".into(), ui.font.clone()));
                    })
                    .observe(
                        |_trigger: Trigger<Pointer<Click>>,
                         mut next: ResMut<NextState<AppState>>| {
                            next.set(AppState::Menu);
                        },
                    );
            });
        });
}

fn exit(mut next: ResMut<NextState<AppState>>) {
    next.set(AppState::Menu);
}
