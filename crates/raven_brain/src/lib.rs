use bevy::prelude::*;

pub mod prelude {
    pub use crate::{BrainPlugin};
}

pub struct BrainPlugin;
impl Plugin for BrainPlugin {
    fn build(&self, _app: &mut App) {
        
        //app.add_plugins(RavenBrainPlugin);
    }
}
