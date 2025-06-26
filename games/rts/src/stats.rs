use crate::prelude::*;

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Gold(pub u32);

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Wood(pub u32);

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Housing(pub u32);
