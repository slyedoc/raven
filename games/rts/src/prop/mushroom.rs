use crate::prelude::*;

/// This is our marker components, so we can keep track of the various in-game entities
#[derive(Component)]
#[require(Name = Name::new("Mushroom"))]
#[require(MinimapIcon = MinimapIcon::new(1.0, tailwind::RED_800.into()))]
#[require(NavMeshAffector)]
//#[require(MinimapIcon(|| MinimapIcon::new(0.5, tailwind::RED_600.into()) ))]
pub struct Mushroom;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MushroomAssets>()
        .add_observer(on_add)
        .add_systems(
            FixedUpdate,
            (spawn_random_mushroom.run_if(on_timer(Duration::from_secs(5))),),
        );
}

// Spawn new mushrooms if there are less than 10
fn spawn_random_mushroom(
    mut commands: Commands,
    mushrooms: Query<Entity, With<Mushroom>>,
    map_size: Res<MapSize>,
) {
    if mushrooms.iter().len() < 30 {
        commands.spawn((Mushroom, Transform::from_translation(map_size.random())));
    }
}

fn on_add(trigger: Trigger<OnAdd, Mushroom>, mut commands: Commands, ass: Res<MushroomAssets>) {
    commands.entity(trigger.target()).insert((
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(ass.material.clone()),
        RigidBody::Static,
        Collider::cuboid(1., 1., 1.),                
        StateScoped(AppState::InGame),
    ));
}

#[derive(Resource)]
struct MushroomAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for MushroomAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Cuboid::new(1., 1., 1.).mesh());

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material = materials.add(StandardMaterial {
            base_color: tailwind::RED_800.into(),
            ..default()
        });

        Self { mesh, material }
    }
}
