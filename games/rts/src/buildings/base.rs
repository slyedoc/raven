use crate::{prelude::*, states::in_game_over::{InGameOver, Winner}};

pub fn plugin(app: &mut App) {
    app.init_resource::<BaseAssets>()
        .add_observer(on_add_base)
        .add_observer(on_remove_base);
}

#[derive(Component)]
#[require(Building)]
#[require(Health = Health::new(2000.))]
#[require(BarSettings::<Health> = BarSettings::<Health> {
    offset: 4.,
    width: 4.,
    ..default()
})]
#[require(NavMeshAffector = NavMeshAffector(None))]
pub struct Base;

pub fn on_add_base(
    trigger: Trigger<OnAdd, Base>,
    mut commands: Commands,
    query: Query<&Team>,
    ass: Res<BaseAssets>,
    team_ass: Res<TeamAssets>,
) {
    let e = trigger.target();
    let team = query.get(e).unwrap();
    commands.entity(e).insert((
        Name::new("Base"),
        Mesh3d(ass.mesh.clone()),
        MeshMaterial3d(team.material(&team_ass)),
        // TODO: scale this
        //SceneRoot(ass.scene.clone()),
        Collider::cuboid(ass.size.x, ass.size.y, ass.size.z),            
        MinimapIcon::new(1.0, team.color()),
    ));
}

pub fn on_remove_base(
    trigger: Trigger<OnRemove, Base>,
    query: Query<&Team>,
    mut commands: Commands,
    mut next: ResMut<NextState<InGameOver>>,
) {
    let e = trigger.target();
    let team = query.get(e).unwrap();

    commands.insert_resource(Winner {
        team: match team {
            Team::Blue => Team::Red,
            Team::Red => Team::Blue,
        },
    });

    next.set(InGameOver::Active);
}

#[derive(Resource)]
pub struct BaseAssets {
    scene: Handle<Scene>,
    size: Vec3,
    mesh: Handle<Mesh>,
}

impl FromWorld for BaseAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let size = vec3(8.0, 3.0, 8.0);
        let mesh = meshes.add(Cuboid::new(size.x, size.y, size.z).mesh());

        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let scene = asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("blueprints/KB3D_LNB_BldgXLG_C_Dome.glb"));

        BaseAssets { scene, mesh, size }
    }
}
