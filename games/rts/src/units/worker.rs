use crate::prelude::*;

#[derive(Component)]
#[require(Unit)]
#[require(Name = Name::new("Worker"))]
#[require(Health = Health::new(100.))]
#[require(BarSettings::<Health> = BarSettings::<Health> {
    offset: 2.,
    width: 2.,
    ..default()
})]
#[require(MovementAcceleration = MovementAcceleration(30.))]
#[require(Speed = Speed::new(2.))]
#[require(MovementDampingFactor = MovementDampingFactor(0.92))]
// #[require(JumpImpulse(|| JumpImpulse(7.0)))]
// #[require(MaxSlopeAngle(|| MaxSlopeAngle(30.0f32.to_radians())))]
#[require(LockedAxes = LockedAxes::ROTATION_LOCKED)]
pub struct Worker;

pub fn plugin(app: &mut App) {
    app.init_resource::<WorkerAssets>()
        .add_observer(on_add_worker);
}

fn on_add_worker(
    trigger: Trigger<OnAdd, Worker>,
    query: Query<&Team>,
    mut commands: Commands,
    ass: Res<WorkerAssets>,
    team_ass: Res<TeamAssets>,
) {
    let e = trigger.target();
    let team = query.get(e).unwrap();
    commands.entity(e).insert((
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(team.material(&team_ass)),
        Collider::capsule(ass.capsule.x, ass.capsule.y),
        team.collision_layers(),
        RigidBody::Dynamic,
        MinimapIcon::new(1.0, team.color()),
    ));
}

#[derive(Resource)]
struct WorkerAssets {
    capsule: Vec2,
    mesh: Handle<Mesh>,
}

impl FromWorld for WorkerAssets {
    fn from_world(world: &mut World) -> Self {
        let capsule = vec2(0.5, 1.0);

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Capsule3d::new(capsule.x, capsule.y).mesh());

        Self { capsule, mesh }
    }
}
