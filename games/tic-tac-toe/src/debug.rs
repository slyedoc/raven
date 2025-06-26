use bevy::{dev_tools::ui_debug_overlay::*, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_plugins(DebugUiPlugin)
        .add_systems(Update, toggle_overlay);
}

// The system that will enable/disable the debug outlines around the nodes
fn toggle_overlay(input: Res<ButtonInput<KeyCode>>, mut options: ResMut<UiDebugOptions>) {
    if input.just_pressed(KeyCode::Space) {
        options.toggle();
    }
}
