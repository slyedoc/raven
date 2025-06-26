use bevy::prelude::*;

mod dead;
pub use dead::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DeadPostProcessPlugin);
}
