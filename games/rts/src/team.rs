use crate::prelude::*;

#[derive(Component, Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Team {
    #[default]
    Blue,
    Red,
}

impl Team {
    pub fn respawn_point(&self) -> Vec3 {
        match self {
            Team::Red => Vec3::new(-10., 0.0, 0.0),
            Team::Blue => Vec3::new(10., 0.0, 0.0),
        }
    }

    pub fn goal(&self) -> Goal {
        match self {
            Team::Red => Goal::AttackMove(Vec3::new(45., 0.0, 0.0)),
            Team::Blue => Goal::AttackMove(Vec3::new(-45., 0.0, 0.0)),
        }
    }
    pub fn other(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
    pub fn collision_layers(&self) -> CollisionLayers {
        match self {
            Team::Red => CollisionLayers::new(
                GameLayer::TeamRed,
                [
                    GameLayer::TeamRed,
                    GameLayer::TeamBlue,
                    GameLayer::Ground,
                    GameLayer::SensorBlue,
                ],
            ),
            Team::Blue => CollisionLayers::new(
                GameLayer::TeamBlue,
                [
                    GameLayer::TeamBlue,
                    GameLayer::TeamRed,
                    GameLayer::Ground,
                    GameLayer::SensorRed,
                ],
            ),
        }
    }

    pub fn sensors(&self) -> CollisionLayers {
        match self {
            Team::Red => CollisionLayers::new(GameLayer::SensorRed, [GameLayer::TeamBlue]),
            Team::Blue => CollisionLayers::new(GameLayer::SensorBlue, [GameLayer::TeamRed]),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Team::Red => tailwind::RED_800.into(),
            Team::Blue => tailwind::BLUE_800.into(),
        }
    }

    pub fn material(&self, ass: &TeamAssets) -> Handle<StandardMaterial> {
        match self {
            Team::Red => ass.red_material.clone(),
            Team::Blue => ass.blue_material.clone(),
        }
    }

    pub fn minimap_material(&self, ass: &TeamAssets) -> Handle<StandardMaterial> {
        match self {
            Team::Red => ass.minimap_red_material.clone(),
            Team::Blue => ass.minimap_blue_material.clone(),
        }
    }
}

impl From<&Team> for Color {
    fn from(value: &Team) -> Self {
        value.color()
    }
}

#[derive(Resource)]
pub struct TeamAssets {
    minimap_red_material: Handle<StandardMaterial>,
    minimap_blue_material: Handle<StandardMaterial>,
    red_material: Handle<StandardMaterial>,
    blue_material: Handle<StandardMaterial>,
}

impl FromWorld for TeamAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let blue_material = materials.add(StandardMaterial {
            base_color: Team::Blue.color().into(),
            ..default()
        });

        let red_material = materials.add(StandardMaterial {
            base_color: Team::Red.color().into(),
            ..default()
        });

        let minimap_blue_material = materials.add(StandardMaterial {
            base_color: tailwind::BLUE_500.into(),
            unlit: true,
            ..default()
        });

        let minimap_red_material = materials.add(StandardMaterial {
            base_color: tailwind::RED_500.into(),
            unlit: true,
            ..default()
        });

        Self {
            minimap_blue_material,
            minimap_red_material,
            blue_material,
            red_material,
        }
    }
}
