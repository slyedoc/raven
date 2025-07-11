#![allow(unused_imports)]
pub mod game;
pub mod menu;
pub mod loading;
pub mod ui;

use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rand::prelude::*;
use bevy_tweening::TweeningPlugin;
use raven_util::prelude::*;
use cfg_if::cfg_if;
use ui::*;

#[cfg(feature = "dev")] use raven_editor::prelude::*;
#[cfg(feature = "dev")] use clap::Parser;

use crate::game::GamePhase;

#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[clap(short, long)]
    seed: Option<u8>,
}

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    LoadingComplete,
    Menu,
    Game,
}


fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minesweeper".to_string(),
                resolution: (1200., 1200.).into(),
                ..default()
            }),
            ..default()
        }),
        TweeningPlugin,        
        FadePlugin::<AppState>::default(),
        FadePlugin::<GamePhase>::default(),   
        // states
        loading::SplashPlugin,
        menu::MenuPlugin,
        game::GamePlugin,
    ))
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::LoadingComplete)
                .load_collection::<UiAssets>(),
        )
        .init_state::<AppState>();

    cfg_if! {
        // allow setup from args
        if #[cfg(feature = "dev")] {
            
            let args = Args::parse();
            let entryopy = match args.seed {
                Some(x) =>EntropyPlugin::<WyRand>::with_seed([x; 8]),
                None => EntropyPlugin::<WyRand>::new(),
            };
            app
            .init_resource::<GameConfig>()
            .add_plugins((
                entryopy,
                EditorPlugin::default()
            ));

        } else {
                app
                .init_state::<AppState>()
                .add_plugins(
                    EntropyPlugin::<WyRand>::new()
                )
                .init_resource::<GameConfig>();
        }
    }
    app
        // order here hides a warning
        .enable_state_scoped_entities::<InGame>()
        .enable_state_scoped_entities::<AppState>()
        .add_computed_state::<InGame>()
        .add_systems(Startup, setup)
        .register_type::<GameMode>()
        .register_type::<GameConfig>()
        .run();
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]

pub struct InGame;

impl ComputedStates for InGame {
    type SourceStates = AppState;
    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::Game { .. } => Some(InGame),
            _ => None,
        }
    }
}

#[derive(Resource, Reflect, Default, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GameConfig {
    pub mode: GameMode,
}

#[derive(Default, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Beginner,
    #[default]
    Intermediate,
    Expert,
    Custom {
        size: UVec2,
        mines: usize,
    },
}
impl GameMode {
    fn get(&self) -> (UVec2, usize) {
        match self {
            GameMode::Beginner => (UVec2::new(8, 8), 10),
            GameMode::Intermediate => (UVec2::new(16, 16), 40),
            GameMode::Expert => (UVec2::new(24, 24), 99),
            GameMode::Custom { size, mines } => (*size, *mines),
        }
    }
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::Beginner => write!(f, "Beginner"),
            GameMode::Intermediate => write!(f, "Intermediate"),
            GameMode::Expert => write!(f, "Expert"),
            GameMode::Custom { size, mines } => write!(f, "Custom ({:?}, {})", size, mines),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("MainCamera"),
        Camera2d::default(),
        Camera {
            hdr: true,
            clear_color: BACKGROUND_COLOR.into(),
            ..default()
        },
    ));
}
