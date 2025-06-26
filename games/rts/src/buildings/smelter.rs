use crate::prelude::*;

#[derive(Component)]
#[require(Building)]
#[require(Name = Name::new("Smelter"))]
#[require(NavMeshAffector)]
#[allow(dead_code)]
pub struct Smelter;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_smelter);
}

fn on_add_smelter(
    trigger: Trigger<OnAdd, Smelter>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.entity(trigger.target()).insert((
        Mesh3d(meshes.add(Cuboid::new(2., 1., 2.).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: tailwind::SLATE_600.into(),
            ..default()
        })),
        RigidBody::Static,
        Collider::cuboid(2., 1., 2.),        
    ));
}
