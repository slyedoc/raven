use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<FootmenAssets>()
        .add_observer(on_add_footmen);
}

#[derive(Component, Default)]
#[require(Unit)]
#[require(MovementAcceleration = MovementAcceleration(30.))]
#[require(Health = Health::new(100.))]
#[require(BarSettings::<Health> = BarSettings::<Health> {
    offset: 2.35,
    width: 2.,
    ..default()
})]
#[require(Speed = Speed::new(2.))]
#[require(SensorRange = SensorRange(10.0))]
#[require(Attack =  Attack::new(60., 10.0, 3.0))]
#[require(BarSettings::<Attack> = BarSettings::<Attack> {
    offset: 1.75,
    width: 1.5,
    ..default()
})]
#[require(BarSettings::<PathInfo> = BarSettings::<PathInfo> {
    offset: 2.,
    width: 1.5,
    ..default()
})]
#[require(MovementDampingFactor = MovementDampingFactor(0.92))]
// #[require(JumpImpulse(|| JumpImpulse(7.0)))]
// #[require(MaxSlopeAngle(|| MaxSlopeAngle(30.0f32.to_radians())))]
#[require(RigidBody = RigidBody::Dynamic)]
pub struct Footmen;

fn on_add_footmen(
    trigger: Trigger<OnAdd, Footmen>,
    query: Query<&Team>,
    mut commands: Commands,
    ass: Res<FootmenAssets>,
    team_ass: Res<TeamAssets>,
) {
    let e = trigger.target();
    let Ok(team) = query.get(e) else {
        error!("no team found");
        return;
    };

    commands.entity(e).insert((
        Name::new("Footmen"),
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(team.material(&team_ass)),
        Collider::capsule(ass.capsule.x, ass.capsule.y),
        team.collision_layers(),
        RigidBody::Dynamic,
        MinimapIcon::new(1.0, team.color()),
    ));
}

#[derive(Resource)]
struct FootmenAssets {
    capsule: Vec2,
    mesh: Handle<Mesh>,
}

impl FromWorld for FootmenAssets {
    fn from_world(world: &mut World) -> Self {
        let capsule = vec2(0.5, 2.0);

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Capsule3d::new(capsule.x, capsule.y).mesh());

        Self { capsule, mesh }
    }
}
