mod selected;
pub use selected::*;

mod minimap;
pub use minimap::*;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((selected::plugin, minimap::plugin));
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct MapSize(pub Vec2);

impl MapSize {
    pub fn half(&self) -> Vec2 {
        self.0 / 2.0
    }

    pub fn random(&self) -> Vec3 {
        let half = self.half();
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(-half.x..half.x),
            0.0,
            rng.gen_range(-half.y..half.y),
        )
    }

    pub fn corners(&self) -> [Vec2; 4] {
        let half = self.half();
        [-half, vec2(-half.x, half.y), half, vec2(half.x, -half.y)]
    }
}
