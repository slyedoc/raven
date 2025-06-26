use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ProjectileAssets>()
        .add_observer(on_add_projectile)
        .add_systems(Update, update_projectile);
}

#[derive(Component)]
#[require(Transform)]
#[require(Speed = Speed::new(6.0))]
pub struct Projectile {
    pub target: Entity,
    pub from: Entity,
    pub dmg: f32,
}

fn on_add_projectile(
    trigger: Trigger<OnAdd, Projectile>,
    mut commands: Commands,
    ass: Res<ProjectileAssets>,
) {
    let e = trigger.target();
    commands.entity(e).insert((
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(ass.material.clone()),
    ));
}

fn update_projectile(
    mut query: Query<(Entity, &Projectile, &Speed, &mut Transform)>,
    target: Query<&GlobalTransform, Without<Dead>>,
    mut commands: Commands,
    time: Res<Time<Physics>>,
) {
    for (e, projectile, speed, mut transform) in query.iter_mut() {
        if let Ok(target_transform) = target.get(projectile.target) {
            let direction = target_transform.translation() - transform.translation;
            let distance = direction.length();
            let velocity = direction.normalize() * speed.current * time.delta_secs();

            if distance < 0.1 {
                commands.entity(e).despawn();
                commands.trigger_targets(
                    Damage {
                        amount: projectile.dmg,
                        from: projectile.from,
                    },
                    projectile.target,
                );
            } else {
                transform.translation += velocity;
            }
        } else {
            // target is dead or missing, destroy the projectile
            commands.entity(e).despawn();
        }
    }
}

#[derive(Resource)]
struct ProjectileAssets {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}

impl FromWorld for ProjectileAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Sphere::new(0.5).mesh());

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material = materials.add(StandardMaterial {
            base_color: tailwind::CYAN_100.into(),
            unlit: true,
            ..default()
        });

        Self { mesh, material }
    }
}
