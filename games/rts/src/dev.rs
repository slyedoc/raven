use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app
    .add_systems(
        Update,
        (
            cycle_debug.run_if(input_just_pressed(KeyCode::Space)),
            charge.run_if(input_just_pressed(KeyCode::Space)),
            spawn_test.run_if(input_just_pressed(KeyCode::KeyZ)),
            toggle_layers.run_if(input_just_pressed(KeyCode::KeyL)),
            (
                spawn_map_tests.run_if(input_just_pressed(KeyCode::KeyM)),
                //update_map_tests,
            )
                .chain(),
            //draw_map_tests,
            draw_team.run_if(in_editor),
        )
            .run_if(in_state(AppState::InGame)),
    );
}

fn cycle_debug(mut debug_mode: ResMut<NavDebugMode>) {
    *debug_mode = match *debug_mode {
        NavDebugMode::Disabled => NavDebugMode::Mesh,
        NavDebugMode::Mesh => NavDebugMode::Wireframe,
        NavDebugMode::Wireframe => NavDebugMode::Disabled,
    };
    info!("Debug mode: {:?}", *debug_mode.as_ref());
}

fn spawn_test(mut commands: Commands) {
    commands
        .spawn((Transform::from_xyz(0., 2., 0.), Footmen, Team::Red))
        .trigger(Team::Red.goal());
}

fn charge(mut commands: Commands, query: Query<(Entity, &Team), (With<Unit>, Without<Worker>)>) {
    for (e, team) in query.iter() {
        commands.entity(e).trigger(team.goal());
    }
}

fn toggle_layers(
    mut commands: Commands,
    query: Single<Entity, With<GameCamera>>,
    mut layer: Local<usize>,
) {
    *layer += 1;
    *layer %= 2;

    let e = *query;
    commands.entity(e).insert(RenderLayers::layer(*layer));
}

fn draw_team(query: Query<(&Transform, &Team)>, mut gismos: Gizmos) {
    for (trans, team) in query.iter() {
        gismos.line(
            trans.translation,
            trans.translation + Vec3::Y * 4.0,
            team.color(),
        );
    }
}

#[derive(Component, Default)]
pub struct MapDebug {
    result: Option<Vec3>,
}

#[derive(Component, Default)]
pub struct QueueMapDebug;

fn spawn_map_tests(
    mut commands: Commands,
    query: Query<Entity, With<MapDebug>>,
    map_size: Res<MapSize>,
    mut toggle: Local<bool>,
) {
    // clear existing
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    *toggle = !*toggle;
    if !*toggle {
        return;
    }

    // spawn debug points to sample the navmesh
    let half = map_size.half();
    let sample = 100;
    for x in linspace::<f32>(-half.x, half.x, sample) {
        for z in linspace::<f32>(-half.y, half.y, sample) {
            commands.spawn((
                Transform::from_xyz(x, 0.1, z),
                MapDebug { result: None },
                QueueMapDebug,
            ));
        }
    }
}

// Since navmesh might not be built yet, we need to check if it is
// fn update_map_tests(
//     mut commands: Commands,
//     mut query: Query<(Entity, &Transform, &mut MapDebug), With<QueueMapDebug>>,
//     navmesh_query: Query<(&NavMeshWrapper, Ref<NavMeshStatus>)>,
//     mut navmeshes: ResMut<Assets<NavMesh>>,
// ) {
//     let (navmesh_handle, status) = navmesh_query.single();
//     if *status != NavMeshStatus::Built {
//         return;
//     }
//     let Some(navmesh) = navmeshes.get_mut(&navmesh_handle.0) else {
//         return;
//     };

//     // navmesh is ready, see if it exists in navmesh
//     for (e, trans, mut debug) in query.iter_mut() {
//         let pos = vec2(trans.translation.x, trans.translation.z);
//         let start_opt = navmesh
//             .get()
//             .get_closest_point_towards(pos, pos + vec2(0.0, -1.0))
//             .map(|p| vec3(p.position().x, 0.1, p.position().y));
//         debug.result = start_opt;
//         commands.entity(e).remove::<QueueMapDebug>();
//     }
// }

// fn draw_map_tests(query: Query<(&Transform, &MapDebug, Has<QueueMapDebug>)>, mut gismos: Gizmos) {
//     for (trans, debug, queued) in query.iter() {
//         let color: Color = match (debug.result, queued) {
//             (None, true) => tailwind::YELLOW_600.into(),
//             (None, false) => tailwind::RED_600.into(),
//             (Some(start), false) => {
//                 gismos.line(trans.translation, start, tailwind::CYAN_600);
//                 tailwind::GREEN_600.into()
//             }
//             (Some(_), true) => unreachable!(),
//         };
//         gismos.line(trans.translation, trans.translation + Vec3::Y, color);
//     }
// }
