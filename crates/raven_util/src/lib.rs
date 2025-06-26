mod bars;
mod camera_free;
mod despawn;
mod gizmos;
mod linspace;
mod fade;

pub mod prelude {
    pub use crate::{bars::*, camera_free::*, despawn::*, gizmos::*, linspace::*,fade::*};
}

// These should be simple qol functionalities