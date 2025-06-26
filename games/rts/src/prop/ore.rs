use raven_nav::prelude::*;

use crate::prelude::*;

/// This is our marker components, so we can keep track of the various in-game entities
#[derive(Component)]
#[require(MinimapIcon = MinimapIcon::new(1.0, tailwind::GRAY_800.into()))]
#[require(NavMeshAffector)]
pub struct Ore;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<OreAssets>()
        .add_observer(on_add)
        .add_systems(
            FixedUpdate,
            (spawn_random_ore.run_if(on_timer(Duration::from_secs_f32(0.2))),),
        );
}

fn on_add(trigger: Trigger<OnAdd, Ore>, mut commands: Commands, ass: Res<OreAssets>) {
    commands.entity(trigger.target()).insert((
        Name::new("Ore"),
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(ass.material.clone()),
        RigidBody::Static,
        Collider::cuboid(1., 1., 1.),        
        StateScoped(AppState::InGame),
    ));
}

// Spawn new mushrooms if there are less than 10
fn spawn_random_ore(
    mut commands: Commands,
    ores: Query<Entity, With<Ore>>,
    map_size: Res<MapSize>,
) {
    if ores.iter().len() < 10 {
        commands.spawn((Ore, Transform::from_translation(map_size.random())));
    }
}

#[derive(Resource)]
struct OreAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for OreAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Cuboid::new(1., 1., 1.).mesh());

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material = materials.add(StandardMaterial {
            base_color: tailwind::GRAY_500.into(),
            ..default()
        });

        Self { mesh, material }
    }
}
