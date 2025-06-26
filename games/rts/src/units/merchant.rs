use crate::prelude::*;

#[derive(Component)]
#[require(Unit)]
#[require(MinimapIcon = MinimapIcon::new(1.0, tailwind::YELLOW_500.into()))]
pub struct Merchant;

pub fn plugin(app: &mut App) {
    app.init_resource::<MerchantAssets>().add_observer(on_add);
}

fn on_add(trigger: Trigger<OnAdd, Merchant>, mut commands: Commands, ass: Res<MerchantAssets>) {
    commands.entity(trigger.target()).insert((
        Name::new("Merchant"),
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(ass.material.clone()),
        Collider::capsule(ass.capsule.x, ass.capsule.y),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,        
        Unit,
    ));
}

#[derive(Resource)]
struct MerchantAssets {
    capsule: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for MerchantAssets {
    fn from_world(world: &mut World) -> Self {
        let capsule = vec2(0.5, 2.0);
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Capsule3d::new(capsule.x, capsule.y).mesh());

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material = materials.add(StandardMaterial {
            base_color: tailwind::YELLOW_500.into(),
            ..default()
        });

        Self {
            capsule,
            mesh,
            material,
        }
    }
}
