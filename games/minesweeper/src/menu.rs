use crate::{AppState, GameConfig, GameMode, ui::*};

use bevy::{ecs::{component::HookContext, spawn::SpawnIter, world::DeferredWorld}, prelude::*};
use bevy_spawn_observer::SpawnObserver;
use raven_util::prelude::*;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .add_systems(OnEnter(MenuState::Main), setup_main)
            .add_systems(OnEnter(MenuState::Settings), setup_settings)
            .add_systems(
                Update,
                update_settings.run_if(in_state(MenuState::Settings)),
            ).add_observer(on_mode_changed);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Menu)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
}

fn setup_main(mut commands: Commands, _asset_server: Res<AssetServer>, ui: Res<UiAssets>) {
    let _font = ui.font.clone();

//  commands.spawn((
//         Name::new("Menu"),
//         StateScoped(MenuState::Main),
//         Node {
//             width: Val::Percent(100.0),
//             height: Val::Percent(100.0),
//             display: Display::Grid,
//             grid_template_rows: vec![
//                 // Menu bar
//                 RepeatedGridTrack::auto(1),
//                 // Property panel
//                 RepeatedGridTrack::fr(1, 1.0),
//                 // Status bar
//                 RepeatedGridTrack::auto(1),
//             ],
//             ..default()
//         },
//         Pickable::IGNORE,
//         children![
//             (
//                 Name::new("Menu Bar"),
//                 Node {
//                     padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
//                     column_gap: Val::Px(5.0),
//                     ..default()
//                 },
//                 BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
//                 children![
//                     //button("Load Scene", spawn_load_scene_modal),
                    
//                 ]
//             ),
//             (
//                 Name::new("Property Panel"),
//                 Node {
//                     width: Val::Px(300.0),
//                     justify_self: JustifySelf::Start,
//                     flex_direction: FlexDirection::Column,
//                     ..default()
//                 },
//                 BackgroundColor(Color::srgb(0.3, 0.1, 0.1)),
//                 children![
//                     (Name::new("Title"), Text::new("Minesweeper"), H1,),
//                     (
//                         Name::new("New Game Button"),
//                         MenuButton,
//                         Children::spawn((
//                             Spawn((
//                                 MenuButtonInner,
//                                 //ImageNode::new(asset_server.load("textures/icon/white/plus.png")),
//                                 Text("New Game".to_string()),
//                             )),
//                             SpawnObserver::new(
//                                 |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
//                                     commands.send_event(FadeTo(AppState::Game));
//                                 },
//                             ),
//                         )),
//                     ),
//                      (
//                         Name::new("Settings Button"),
//                         MenuButton,
//                         Children::spawn((
//                             Spawn((MenuButtonInner, Text::new("Settings"))),
//                             SpawnObserver::new(
//                                 |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
//                                     commands.set_state(MenuState::Settings);
//                                 },
//                             ),
//                             //ImageNode::new(asset_server.load("textures/icon/white/checkmark.png")),
//                         )),
//                     ),
//                     #[cfg(not(target_arch = "wasm32"))]
//                     (
//                         Name::new("Exit Button"),
//                         MenuButton,
//                         Children::spawn((
//                             Spawn((MenuButtonInner, Text::new("Exit"))),
//                             SpawnObserver::new(
//                                 |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
//                                     commands.send_event(AppExit::Success);
//                                 },
//                             ),
//                             //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
//                         )),
//                     )
//                 ]
//             ),
//             (
//                 Name::new("Status Bar"),
//                 Node {
//                     display: Display::Flex,
//                     justify_content: JustifyContent::SpaceBetween,
//                     padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
//                     ..default()
//                 },
//                 BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
//                 children![
//                     status_bar_text("Status Bar"),
//                     status_bar_text("v0.1.0")
//                 ],
//             )
//         ],
//     ));

    commands
        .spawn((
            Name::new("Menu Panel"),
            StateScoped(MenuState::Main),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(20.),
                top: Val::Percent(10.),
                height: Val::Percent(80.),
                width: Val::Percent(60.),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor(PANEL_BACKGROUND),
            BorderColor(PANEL_BORDER),
            // Outline {
            //     width: Val::Px(2.),
            //     color: Color::WHITE,
            //     ..default()
            // },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(30.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Stretch,
                    flex_direction: FlexDirection::Column,

                    ..default()
                },
                children![
                    (Name::new("Title"), Text::new("Minesweeper"), H1,),
                    (
                        Name::new("New Game Button"),
                        MenuButton,
                        Children::spawn((
                            Spawn((
                                MenuButtonInner,
                                //ImageNode::new(asset_server.load("textures/icon/white/plus.png")),
                                Text("New Game".to_string()),
                            )),
                            SpawnObserver::new(
                                |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                                    commands.send_event(FadeTo(AppState::Game));
                                },
                            ),
                        )),
                    ),
                    (
                        Name::new("Settings Button"),
                        MenuButton,
                        Children::spawn((
                            Spawn((MenuButtonInner, Text::new("Settings"))),
                            SpawnObserver::new(
                                |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                                    commands.set_state(MenuState::Settings);
                                },
                            ),
                            //ImageNode::new(asset_server.load("textures/icon/white/checkmark.png")),
                        )),
                    ),
                    #[cfg(not(target_arch = "wasm32"))]
                    (
                        Name::new("Exit Button"),
                        MenuButton,
                        Children::spawn((
                            Spawn((MenuButtonInner, Text::new("Exit"))),
                            SpawnObserver::new(
                                |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                                    commands.send_event(AppExit::Success);
                                },
                            ),
                            //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
                        )),
                    )
                ],
            ));
        });

}

fn status_bar_text(text: impl Into<String>) -> impl Bundle {
    (
        Text::new(text),
        TextFont::from_font_size(15.0),
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    )
}

fn setup_settings(mut commands: Commands, ui: Res<UiAssets>,) {
    let _font = ui.font.clone();

    let settings_panel = commands
        .spawn((
            Name::new("Settings"),
            StateScoped(MenuState::Settings),
            Node {
                //position_type: PositionType::Absolute,
                left: Val::Percent(20.),
                top: Val::Percent(10.),
                width: Val::Percent(60.),
                height: Val::Percent(80.),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor(PANEL_BACKGROUND),
            BorderColor(PANEL_BORDER),
            // Outline {
            //     width: Val::Px(2.),
            //     color: Color::WHITE,
            //     ..default()
            // },
        ))
        .id();

    commands.spawn((
        ChildOf(settings_panel),
        Node {
            padding: UiRect::all(Val::Px(30.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Children::spawn((
            Spawn((Name::new("Title"), Text::new("Settings"), H1)),
            SpawnIter(
                [GameMode::Beginner, GameMode::Intermediate, GameMode::Expert]
                    .into_iter()
                    .map(|m| {
                        let (size, mines) = m.get();
                        (
                            Name::new(format!("{}", &m)),
                            Node {
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            children![
                                (
                                    MenuButton,
                                    ModeButton(m),
                                    Children::spawn((
                                        Spawn((MenuButtonInner, Text::new(format!("{}", &m)))),
                                        SpawnObserver::new(move |trigger: Trigger<Pointer<Click>>, mut config: ResMut<GameConfig>, mut commands: Commands| {
                                                config.mode = m;     
                                                commands.trigger_targets(ModeChanged, trigger.target());
                                            },
                                        ),
                                    ))
                                ),
                                (Name::new("Text"), Text::new(format!("{}x{} {} mines", size.x, size.y, mines)), H4,),
                            ],
                        )
                    }),
            ),
        )),
    ));

    // let row = commands
    //     .spawn((
    //         ChildOf(settings_panel),
    //         Node {
    //             flex_direction: FlexDirection::Row,
    //             ..default()
    //         },
    //         children![(
    //             Text::new("Volume"),
    //             //ImageNode::new(asset_server.load("textures/icon/white/plus.png")),
    //         ),],
    //     ))
    //     .id();

    // commands
    //     .spawn((ChildOf(row), Slider { value: 0.5 }))
    //     .observe(
    //         |trigger: Trigger<SliderChanged>,
    //          mut music_controller: Single<&mut AudioSink, With<BackgroundMusic>>| {
    //             music_controller.set_volume(Volume::Linear(trigger.event().value));
    //         },
    //     );

    commands
        .spawn((
            ChildOf(settings_panel),
            Name::new("Exit Button"),
            MenuButton,
            children!((
                MenuButtonInner,
                Text::new("Exit"),
                //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
            )),
        ))
        .observe(
            |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                commands.set_state(MenuState::Main);
            },
        );

    commands.spawn((
        Name::new("Version Text"),
        StateScoped(MenuState::Settings),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.),
            bottom: Val::Px(5.),
            ..default()
        },
        children!((
            Text("Version: 0.1.0".to_string()),
            //ImageNode::new(asset_server.load("textures/icon/white/exitRight.png")),
        )),
    ));
}

#[derive(Component)]
#[component(on_add = on_add_mode_button)]
pub struct ModeButton(pub GameMode);

fn on_add_mode_button(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let config_mode = world.resource::<GameConfig>().mode;
    let m = world.get::<ModeButton>(entity).unwrap().0;

    if m == config_mode {
        world
            .commands()
            .entity(entity)
            .insert(Focus);
    }    
}

#[derive(Event)]
pub struct ModeChanged;

fn on_mode_changed(
    trigger: Trigger<ModeChanged>,
    mut commands: Commands,
    query: Query<Entity, (With<ModeButton>, With<Focus>)>,
) {
    for e in query.iter() {
        commands.entity(e).remove::<Focus>();
    }
    commands.entity(trigger.target()).insert(Focus);
}

fn update_settings(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.set_state(MenuState::Main);
    }
}
