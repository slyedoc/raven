use bevy::{ecs::query, prelude::*};

use raven_util::prelude::*;

use super::AppState;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(AppState::Loading), setup)
            .add_systems(OnEnter(AppState::LoadingComplete), go_to_menu)
            .add_systems(OnExit(AppState::Loading), cleanup);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnLoadingScreen;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");
    // Display the logo
    commands.spawn((
        OnLoadingScreen,
        StateScoped(AppState::Loading),
        ui_root("SplashScreen"),
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: Val::Px(200.0),
                ..default()
            },
        )],
    ));
}

fn cleanup(mut commands: Commands, mut query: Query<Entity, With<OnLoadingScreen>>) {
    // Despawn all entities tagged with `OnLoadingScreen`
    for e in query.iter_mut() {
        commands.entity(e).despawn();
    }
}

fn go_to_menu(mut commands: Commands) {
    // Remove the splash screen
    commands.send_event(FadeTo(AppState::Menu));
}
