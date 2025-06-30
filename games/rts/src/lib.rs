#![allow(warnings)]
#![feature(iter_array_chunks)]
mod ai;
mod buildings;
#[cfg(feature = "dev")]
mod dev;
mod prop;
mod resources;
mod shader;
mod states;
mod stats;
mod team;
mod ui;
mod units;

use bevy::{math::bounding::RayCast3d, window::WindowResolution};

pub use team::*;

use bevy_rand::prelude::*;
#[cfg(feature = "dev")]
use raven_bvh::prelude::*;
use raven_editor::prelude::*;
use raven_nav::prelude::*;

#[allow(unused_imports)]
mod prelude {
    pub use crate::{
        ai::*, buildings::*, prop::*, resources::*, shader::*, states::*, stats::*, team::*, ui::*,
        units::*, *,
    };
    pub use avian3d::prelude::*;
    pub use bevy::{
        asset::RenderAssetUsages,
        color::palettes::*,
        core_pipeline::tonemapping::Tonemapping,
        ecs::entity::EntityHashMap,
        input::common_conditions::*,
        math::{vec2, vec3, VectorSpace},
        pbr::NotShadowCaster,
        picking::prelude::*,
        prelude::*,
        render::{
            camera::RenderTarget,
            render_resource::{
                Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
            },
            view::{NoFrustumCulling, RenderLayers},
        },
        time::common_conditions::*,
        window::PrimaryWindow,
    };
    pub use rand::*;
    use raven_util::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::{f32::consts::*, fmt::Debug, hash::Hash, marker::PhantomData, time::Duration};
}
use css::BISQUE;
use prelude::*;
use raven_util::prelude::*;

use crate::states::in_paused::InPaused;

// DEFAULT_LAYERS 0
pub const MINIMAP_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Ground,
    TeamRed,
    TeamBlue,
    SensorRed,
    SensorBlue,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ActivePlayer;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Default)]
pub struct RtsPlugin {
    pub seed: Option<u8>,
}

impl Plugin for RtsPlugin {
    fn build(&self, app: &mut App) {
        // setup players
        for team in [Team::Red, Team::Blue].iter() {
            let p = app.world_mut().spawn((
                Name::new(format!("Player {:?}", team)),
                Player,
                team.clone()
            )).id();
            
            if *team == Team::Red {
                app.world_mut().entity_mut(p).insert(ActivePlayer);
            }
        }

        app.add_plugins((
            MeshPickingPlugin,
            PhysicsPlugins::default(),
            NavPlugin,
            bevy_skein::SkeinPlugin {
                handle_brp: cfg!(feature = "dev"),
            },
            CameraFreePlugin,
            //VleueNavigatorPlugin,
            //NavmeshUpdaterPlugin::<Collider, Obstacle>::default(),
            match self.seed {
                Some(x) => EntropyPlugin::<WyRand>::with_seed([x; 8]),
                None => EntropyPlugin::<WyRand>::new(),
            },
            #[cfg(feature = "dev")]
            EditorPlugin::default(),
            #[cfg(feature = "dev")]
            dev::plugin,
        ));
        //#[cfg(feature = "dev")] app.add_plugins(EditorPlugin::default());
        app.add_plugins((
            BarPlugin::<Health>::default(),
            BarPlugin::<Attack>::default(),
            BarPlugin::<PathInfo>::default(),
            resources::plugin,
            buildings::plugin,
            units::plugin,
            prop::plugin,
            ui::plugin,
            ai::plugin,
            shader::plugin,
        ))
        .init_resource::<TeamAssets>()
        .insert_resource(
            ColorScheme::<Attack>::new().foreground_color(ForegroundColor::Static(BISQUE.into())),
        )
        .insert_resource(
            ColorScheme::<PathInfo>::new()
                .foreground_color(ForegroundColor::Static(tailwind::YELLOW_400.into())),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Physics>::default().with_relative_speed(1.0))
        .insert_resource(MapSize(vec2(100.0, 40.0)))
        // See `AppState` for more setup
        .register_type::<GameCamera>()
        .add_systems(Update, ray_cast)
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_plugins((
            FadePlugin::<AppState>::default(),
            states::in_game::plugin, // game starts here
            states::in_paused::plugin,
            states::in_game_over::plugin,
            in_scoreboard::plugin,
            menu::plugin,
        ));
    }
}

fn ray_cast(
    camera_query: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
    window: Single<&Window>,
    tlas_query: Single<Entity, (With<Nav>, With<Tlas>)>,
    tlas: TlasCast,
    mut gizmos: Gizmos,
    mut nav_path: NavPath,
    input: Res<ButtonInput<MouseButton>>,
    mut start_pos: Local<Vec3>,
    mut end_pos: Local<Vec3>,
) {
    let (camera, camera_transform) = *camera_query;
    let tlas_entity = *tlas_query;

    // Use Right mouse buttons to set start
    if input.pressed(MouseButton::Right) {
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        let ray_cast = RayCast3d::from_ray(ray, 100.0);
        if let Some((_e, hit)) = tlas.intersect_tlas(&ray_cast, tlas_entity) {
            *start_pos = ray.get_point(hit.distance);
        }
    }

    // Use Left mouse button to set end
    if input.pressed(MouseButton::Left) {
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let ray_cast = RayCast3d::from_ray(ray, 100.0);
        if let Some((_e, hit)) = tlas.intersect_tlas(&ray_cast, tlas_entity) {
            *end_pos = ray.get_point(hit.distance);
        }
    }

    gizmos.sphere(*start_pos, 0.1, tailwind::GREEN_400);
    gizmos.sphere(*end_pos, 0.1, tailwind::RED_400);
    gizmos.line(*start_pos, *end_pos, tailwind::YELLOW_400);

    // Run pathfinding to get a polygon path.
    match nav_path.find_path(tlas_entity, *start_pos, *end_pos, None, Some(&[1.0, 0.5])) {
        Ok(path) => gizmos.linestrip(path, tailwind::BLUE_300),
        Err(error) => error_once!("Error with pathfinding: {:?}", error),
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GameCamera;

pub fn pause(mut next: ResMut<NextState<InPaused>>) {
    next.set(InPaused::Active);
}
