mod minimap;
mod toolbar;

use raven_bvh::prelude::{SpawnBvhForTlas, TlasCamera};
use strum_macros::EnumIter;
use raven_nav::prelude::*;

use crate::{prelude::*, states::in_paused::InPaused};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::InGame),
        (
            // See setup_game in main
            // ui
            toolbar::create_toolbar,
            minimap::create_minimap,
            ui::create_select,
            ui::create_quick,
            setup_game,
        ),
    )        
    .add_systems(
        Update,
        (ui::update_selected_list.run_if(resource_changed::<Selected>),),
    )
    .add_systems(
        Update,
        (            
            despawn_by_y::<-100>, // despawn entities that fall below a certain height            
            pause.run_if(input_just_pressed(KeyCode::Escape).and(in_state(InPaused::Disabled))),                        
        ).run_if(in_state(AppState::InGame)),
    );
}


fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    map_size: Res<MapSize>,
    mut time: ResMut<Time<Physics>>,
) {
    time.unpause();

    let tlas = commands.spawn((
        Name::new("Nav"),
        Nav::new(0.5, 2.0, Vec3::new(map_size.x, 20.0, map_size.y) ),        
        StateScoped(AppState::InGame),
    )).id();


    commands.spawn((
        Name::new("MainCamera"),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()            
        },
        //TlasCamera::new(512, 512, tlas),
        GameCamera,
        Transform::from_xyz(0., 10., 40.).looking_at(Vec3::ZERO, Vec3::Y),        
        CameraFree,
        StateScoped(AppState::InGame),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        StateScoped(AppState::InGame),
    ));

    commands
        .spawn((
            Name::new("Ground"),
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, map_size.half()).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: tailwind::GRAY_400.into(),
                ..default()
            })),
            Transform::default(),
            Collider::cuboid(map_size.x, 0.01, map_size.y),
            NavMeshAffector::default(),      
            UpdateTileAffectors,      
            RigidBody::Static,
            //CollisionLayers::new(GameLayer::Ground, [GameLayer::TeamBlue, GameLayer::TeamRed]),
            StateScoped(AppState::InGame),
        ))
        .observe(
            |trigger: Trigger<Pointer<Click>>, mut commands: Commands, selected: Res<Selected>| {
                let e = trigger.event();
                if let Some(pos) = e.hit.position {
                    if selected.0.len() > 0 {
                        // TODO: Grid based movement
                        commands.trigger_targets(
                            Goal::Move(vec3(pos.x, 0.0, pos.z)),
                            selected.0.clone(),
                        );
                    }
                }
            },
        );

    
    for team in [Team::Red, Team::Blue].iter() {
        let offset = match team {
            Team::Red => -1.0,
            Team::Blue => 1.0,
        };

        commands.spawn((
            Name::new("Tower"),
            Tower,
            team.clone(),
            Transform::from_xyz(offset * 20.0, 2.0, 0.),
        ));

        commands.spawn((
            team.clone(),
            Spawner::<Footmen>::default(),
            Transform::from_xyz(offset * 15.0, 1.0, 0.),
        ));

        commands.spawn((
            team.clone(),
            Footmen,
            Transform::from_xyz(offset * 15.0, 1.0, 0.),
        ));

        commands.spawn((
            team.clone(),
            Worker,
            Transform::from_xyz(offset * 18.0, 1.0, 0.),
        ));

        commands.spawn((
            Base,
            team.clone(),
            Transform::from_xyz(offset * 30., 1.5, 0.),
        ));
    }

    // commands.spawn((
    //     Smelter,
    //     Transform::from_translation(Vec3::new(0.0, 0., 10.0)),
    // ));

    // commands.spawn((
    //     Merchant,
    //     Transform::from_translation(Vec3::new(-10.0, 2., -5.0)),
    // ));

    // Begin with three mushrooms our miner can eat
    for _i in 0..30 {
        commands.spawn((Mushroom, Transform::from_translation(map_size.random())));
    }

    // Spawn 10 ores we can mine as well
    for _i in 0..30 {
        commands.spawn((Ore, Transform::from_translation(map_size.random())));
    }
}

#[derive(Component, EnumIter, Debug)]
enum GameAction {
    Select,
    Move,
    Attack,
    Build,
    Harvest,
    Stop,
}

mod ui {
    use strum::IntoEnumIterator;

    use crate::{prelude::*, states::in_paused::InPaused};

    use super::GameAction;

    #[derive(Component)]
    pub struct SelectedListText;

    pub fn create_select(mut commands: Commands, ui: Res<UiAssets>) {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(25.0),
                    bottom: Val::Percent(5.0),
                    width: Val::Percent(50.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(6.0)),
                    //row_gap: MARGIN,
                    ..default()
                },
                BackgroundColor(UI_BG),
                StateScoped(AppState::InGame),
            ))
            .with_children(|b| {
                b.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        ..default()
                    },
                    SelectedListText,
                ));

                b.spawn((Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(200.0),
                    ..default()
                },))
                    .with_children(|b| {
                        for chunk in GameAction::iter().array_chunks::<3>() {
                            b.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                ..default()
                            })
                            .with_children(|b| {
                                for a in chunk {
                                    b.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(50.),
                                            height: Val::Px(50.),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            padding: UiRect::all(MARGIN),
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
                                        b.spawn(create_button_text(
                                            format!("{:?}", a),
                                            ui.font.clone(),
                                        ));
                                    });
                                }
                            });
                        }
                    });
            });
    }

    pub fn update_selected_list(
        mut commands: Commands,
        list: Res<Selected>,
        query: Query<(Entity, Option<&Children>), With<SelectedListText>>,
        ui: Res<UiAssets>,
    ) {
        for (e, children) in query.iter() {
            // clear current
            if let Some(children) = children {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }

            let chunk_size = 5;
            for chuck in list.chunks(chunk_size) {
                commands.entity(e).with_children(|b| {
                    b.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    })
                    .with_children(|b| {
                        for i in 0..chuck.len() {
                            // target
                            b.spawn(create_button_outer()).with_children(|b| {
                                b.spawn(create_button_text(format!("T {i}"), ui.font.clone()));
                            });
                            // .observe(
                            //     |_trigger: Trigger<Pointer<Down>>, mut selected: ResMut<NextState<InPaused>>| {
                            //         next.set(InPaused::Active);
                            //     },
                            // );
                        }
                    });
                });
            }
        }
    }

    pub fn create_quick(mut commands: Commands, ui: Res<UiAssets>) {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(5.),
                    bottom: Val::Percent(5.),
                    width: Val::Percent(10.),
                    height: Val::Px(200.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(6.0)),
                    //row_gap: MARGIN,
                    ..default()
                },
                BackgroundColor(UI_BG),
                StateScoped(AppState::InGame),
            ))
            .with_children(|b| {
                // Pause
                b.spawn(create_button_square())
                    .with_children(|b| {
                        // emjoi for pause is ⏸
                        b.spawn(create_button_text("P".into(), ui.font.clone()));
                    })
                    .observe(
                        |_trigger: Trigger<Pointer<Click>>,
                         mut next: ResMut<NextState<InPaused>>| {
                            next.set(InPaused::Active);
                        },
                    );
            });
    }
}
