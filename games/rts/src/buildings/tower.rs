use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<TowerAssets>()
        .add_observer(on_add_tower);
}

#[derive(Component, Debug)]
#[require(Building)]
#[require(Name = Name::new("Tower"))]
#[require(Health = Health::new(1000.))]
#[require(BarSettings::<Health> = BarSettings::<Health> {
    offset: 3.5,
    width: 3.,
    ..default()
})]
#[require(SensorRange = SensorRange(12.))]
#[require(Attack = Attack::new(50.0, 12., 2.0))]
#[require(BarSettings::<Attack> = BarSettings::<Attack> {
    offset: 3.0,
    width: 2.,
    ..default()
})]
#[require(Aggro)]
#[require(NavMeshAffector)]
pub struct Tower;

fn on_add_tower(
    trigger: Trigger<OnAdd, Tower>,
    mut commands: Commands,
    query: Query<&Team>,
    ass: Res<TowerAssets>,
    team_ass: Res<TeamAssets>,
) {
    let e = trigger.target();
    let team = query.get(e).unwrap();
    commands.entity(e).insert((
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(team.material(&team_ass)),
        ColliderConstructor::Capsule {
            radius: ass.capsule.x,
            height: ass.capsule.y,
        },
        RigidBody::Static,
        MinimapIcon::new(2.0, team.color()),
    ));
}

#[derive(Resource)]
struct TowerAssets {
    capsule: Vec2,
    mesh: Handle<Mesh>,
}

impl FromWorld for TowerAssets {
    fn from_world(world: &mut World) -> Self {
        let capsule = vec2(1.0, 3.0);

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Capsule3d::new(capsule.x, capsule.y).mesh());

        Self { capsule, mesh }
    }
}
