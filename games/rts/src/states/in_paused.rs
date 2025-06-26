use bevy::window::CursorGrabMode;

use crate::{prelude::*, states::in_game_over::{InGameOver, Winner}};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
pub enum InPaused {
    #[default]
    Disabled,
    Active,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_sub_state::<InPaused>()
        .enable_state_scoped_entities::<InPaused>()
        .add_systems(OnEnter(InPaused::Active), setup)
        .add_systems(
            Update,
            exit.run_if(input_just_pressed(KeyCode::Escape).and(in_state(InPaused::Active))),
        )
        .add_systems(OnEnter(InPaused::Disabled), unpause);
}

fn setup(
    mut commands: Commands,
    ui: Res<UiAssets>,
    mut time: ResMut<Time<Virtual>>,
    //mut toggle_cursor_grab: ResMut<ToggleCursorGrab>,
    //mut mouse_cursor_grab: ResMut<MouseCursorGrab>,
    mut windows: Query<&mut Window>,
) {
    //toggle_cursor_grab.0 = false;
    //mouse_cursor_grab.0 = false;

    for mut window in &mut windows {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }

    time.pause();

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
            StateScoped(InPaused::Active),
        ))
        .with_children(|b| {
            b.spawn((
                Node {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(UI_BG),
            ))
            .with_children(|b| {
                b.spawn(Node {
                    padding: UiRect::all(MARGIN),
                    ..default()
                })
                .with_children(|b| {
                    b.spawn((
                        Text::new("Paused"),
                        TextFont {
                            font: ui.font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(tailwind::GRAY_100.into()),
                    ));
                });

                // Resume
                b.spawn((
                    Button,
                    Node {
                        width: Val::Percent(80.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(MARGIN),
                        //margin: UiRect::all(MARGIN),
                        ..default()
                    },
                    Outline {
                        width: OUTLINE_WITHD,
                        color: UI_OUTLINE,
                        ..default()
                    },
                    BackgroundColor(UI_BTN_BG),
                ))
                .with_children(|b| {
                    b.spawn(create_button_text("Resume".into(), ui.font.clone()));
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut next: ResMut<NextState<InPaused>>| {
                        next.set(InPaused::Disabled);
                    },
                );

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
            });
        });
}

fn exit(mut next: ResMut<NextState<InPaused>>) {
    next.set(InPaused::Disabled);
    dbg!("exit paused");
}

fn unpause(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}
