use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup).add_systems(
        Update,
        exit.run_if(input_just_pressed(KeyCode::Escape).and(in_state(AppState::Menu))),
    );
}

fn exit(mut next: ResMut<NextState<AppState>>) {
    next.set(AppState::InGame);
}

fn setup(
    mut commands: Commands,
    ui: Res<UiAssets>,
    #[cfg(feature = "dev")] mut next: ResMut<NextState<AppState>>,
    #[cfg(feature = "dev")] mut skip_first: Local<bool>,
) {
    // skip menu on first load
    #[cfg(feature = "dev")]
    if !*skip_first {
        next.set(AppState::InGame);
        *skip_first = true;
        return;
    }

    commands.spawn((
        Name::new("MainCamera"),
        StateScoped(AppState::Menu),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
    // commands.spawn((
    //     Camera3d::default(),
    //     Camera {
    //         hdr: true,
    //         ..default()
    //     },
    //     Transform::IDENTITY,
    //     Tonemapping::AcesFitted,
    //     GameCamera::Static,
    //     ActiveCamera,
    //     StateScoped(AppState::Menu),
    // ));

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
            StateScoped(AppState::Menu),
        ))
        .with_children(|b| {
            b.spawn((
                Node {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(6.0)),
                    //row_gap: MARGIN,
                    ..default()
                },
                BackgroundColor(UI_BG),
            ))
            .with_children(|b| {
                b.spawn((create_title("Zero RTS", ui.font.clone()),));

                b.spawn(create_button_outer())
                    .with_children(|b| {
                        b.spawn(create_button_text("New Game".into(), ui.font.clone()));
                    })
                    .observe(
                        |_trigger: Trigger<Pointer<Click>>,
                         mut commands: Commands| {
                            commands.send_event(FadeTo(AppState::InGame));
                        },
                    );

                b.spawn(create_button_outer())
                    .with_children(|b| {
                        b.spawn(create_button_text("Exit".into(), ui.font.clone()));
                    })
                    .observe(
                        |_trigger: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>| {
                            app_exit.write(AppExit::Success);
                        },
                    );
            });
        });
}
