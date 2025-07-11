mod bars;
mod camera_free;
mod despawn;
mod fade;
mod gizmos;
mod linspace;
mod theme;

pub mod prelude {
    pub use crate::{
        bars::*,
        camera_free::*,
        despawn::*,
        fade::*,
        gizmos::*,
        linspace::*,
        theme::{ThemePlugin, interaction::InteractionPalette, palette as ui_palette, widget::*},
    };
}

// These should be simple qol functionalities
