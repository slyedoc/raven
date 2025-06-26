mod mushroom;
pub use mushroom::*;

mod ore;
pub use ore::*;

mod projectile;
pub use projectile::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((mushroom::plugin, ore::plugin, projectile::plugin));
}
