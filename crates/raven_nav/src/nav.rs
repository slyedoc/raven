use crate::{agent::*, character::*, tile::*};
use bevy::{
    math::bounding::Aabb3d,
    platform::collections::{HashMap, HashSet},
    prelude::*,
    tasks::Task,
};
use bevy_inspector_egui::{inspector_options::std_options::NumberDisplay, prelude::*};
use raven_bvh::prelude::*;
use std::num::{NonZeroU8, NonZeroU16};

/// A navigation area is a region of the world that can be navigated by agents and characters.
#[derive(Component, Reflect, Debug, Clone, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
#[require(
    Transform,
    NavAgents, // list of agents in the waymap
    NavCharacters, // list of characters in the waymap
    NavTiles, // list of tiles in the waymap
    TileLookup, // lookup of tile entities by their coordinates
    DirtyTiles, // tracks tiles that need to be updated
    NavGenerationTasks, // list of tasks that are currently generating tiles
    AgentOptions, // used for agent avoidance
    Visibility, // used for rendering view mesh
    Tlas,
    TlasRebuildStrategy = TlasRebuildStrategy::Mannual(false),
)]
pub struct Nav {
    /// Extents of the Nav area
    ///
    /// **Suggested value**: As small as possible whilst still keeping the entire world within it.
    ///    
    //#[inspector(min = Vec3::new(1.0, 1.0, 1.0), max = Vec3::new(1000.0, 1000.0, 1000.0))]
    pub world_half_extents: Vec3,

    // Voxelization settings
    /// The horizontal resolution of the voxelized tile.
    ///
    /// **Suggested value**: 1/2 of character radius.
    ///
    /// Smaller values will increase tile generation times with diminishing returns in nav-mesh detail.
    #[inspector(min = 0.1, max = 1.0, speed = 0.1, display = NumberDisplay::Slider)]
    pub cell_width: f32,

    /// The vertical resolution of the voxelized tile.
    ///
    /// **Suggested value**: 1/2 of cell_width.
    ///
    /// Smaller values will increase tile generation times with diminishing returns in nav-mesh detail.
    #[inspector(min = 0.001)]
    pub cell_height: f32,

    /// Length of a tile's side in cells. Resulting size in world units is ``tile_width * cell_width``.
    ///
    /// **Suggested value**: ???
    ///
    /// Higher means more to update each time something within the tile changes, smaller means you will have more overhead from connecting the edges to other tiles & generating the tile itself.
    pub tile_width: NonZeroU16,

    // /// Bottom extents of the world on the Y-axis. The top extents is capped by ``world_bottom_bound + cell_height * u16::MAX``.
    // ///
    // /// **Suggested value**: Minium Y position of anything in the world that should be covered by the nav mesh.
    // pub world_bottom_bound: f32,
    /// Maximum incline/slope traversable when navigating in radians.
    #[inspector(min = 0.0, max = 90.0, speed = 1.0, display = NumberDisplay::Slider)]
    pub max_traversable_slope_degrees: f32,
    /// Minimum open height for an area to be considered walkable in cell_height(s).
    ///
    /// **Suggested value**: The height of character * ``cell_height``, rounded up.
    pub walkable_height: u16,
    /// This will "pull-back" the nav-mesh from edges, meaning anywhere on the nav-mesh will be walkable for a character with a radius of ``walkable_radius * cell_width``.
    ///
    /// **Suggested value**: ``ceil(character_radius / cell_width)`` (2-3 if `cell_width` is 1/2 of ``character_radius``)  
    pub walkable_radius: u16,

    /// Maximum height difference that is still considered traversable in cell_height(s). This smooths out stair steps and small ledges.
    pub step_height: u16,

    /// Minimum size of a region in cells, anything smaller than this will be removed. This is used to filter out smaller disconnected island that may appear on surfaces like tables.
    pub min_region_area: u32,
    /// Maximum size of a region in cells we can merge other regions into.
    pub max_region_area_to_merge_into: u32,

    /// Maximum length of an edge before it's split.
    ///
    /// **Suggested value**: Start high and reduce if there are issues.
    pub max_edge_length: u16,
    /// Maximum difference allowed for simplified contour generation on the XZ-plane in cell_width(s).
    ///
    /// **Suggested value range**: `[1.1, 1.5]`
    pub max_contour_simplification_error: f32,

    /// Max tiles to generate in parallel at once. A value of ``None`` will result in no limit.
    ///
    /// Adjust this to control memory & CPU usage. More tiles generating at once will have a higher memory footprint.
    pub max_tile_generation_tasks: NonZeroU16,

    /// TODO: this breaks when any meshes are concave like heightfields. do not use this yet.
    /// When not None, height correct nav-mesh polygons where the surface height differs too much from the surface in cells. This is very useful for bumpy terrain.
    ///
    /// Helps on bumpy shapes like terrain but comes at a performance cost.
    /// **Experimental**: This may have issues at the edges of regions.
    pub experimental_detail_mesh_generation: Option<DetailMeshSettings>,
}

impl Default for Nav {
    fn default() -> Self {
        Self::new(0.5, 2.0, Vec3::new(100.0, 100.0, 100.0))
    }
}

impl Nav {
    pub fn new(agent_radius: f32, agent_height: f32, size: Vec3) -> Self {
        let cell_width = agent_radius / 2.0;
        let cell_height = agent_radius / 4.0;

        let walkable_height = (agent_height / cell_height) as u16;

        Self {
            world_half_extents: size / 2.0,
            cell_width,
            cell_height,
            tile_width: NonZeroU16::new(120).unwrap(),

            max_traversable_slope_degrees: 50.0,
            walkable_height,
            walkable_radius: 2,
            step_height: 3,
            min_region_area: 100,
            max_region_area_to_merge_into: 500,
            max_edge_length: 80,
            max_contour_simplification_error: 1.1,
            max_tile_generation_tasks: NonZeroU16::new(8).unwrap(),
            experimental_detail_mesh_generation: None,
        }
    }

    /// Setter for [`NavMeshSettings::walkable_radius`]
    pub fn with_walkable_radius(mut self, walkable_radius: u16) -> Self {
        self.walkable_radius = walkable_radius;

        self
    }
    /// Setter for [`NavMeshSettings::tile_width`]
    pub fn with_tile_width(mut self, tile_width: NonZeroU16) -> Self {
        self.tile_width = tile_width;

        self
    }
    /// Setter for [`NavMeshSettings::max_traversable_slope_degrees`]
    pub fn with_traversible_slope(mut self, traversible_slope: f32) -> Self {
        self.max_traversable_slope_degrees = traversible_slope;

        self
    }
    /// Setter for [`NavMeshSettings::max_tile_generation_tasks`]
    pub fn with_max_tile_generation_tasks(mut self, max_tile_generation_tasks: NonZeroU16) -> Self {
        self.max_tile_generation_tasks = max_tile_generation_tasks;

        self
    }
    /// Setter for [`NavMeshSettings::step_height`]
    pub fn with_step_height(mut self, step_height: u16) -> Self {
        self.step_height = step_height;

        self
    }
    /// Setter for [`NavMeshSettings::min_region_area`] & [`NavMeshSettings::max_region_area_to_merge_into`]
    pub fn with_region_area(
        mut self,
        min_region_area: u32,
        max_region_area_to_merge_into: u32,
    ) -> Self {
        self.min_region_area = min_region_area;
        self.max_region_area_to_merge_into = max_region_area_to_merge_into;

        self
    }
    /// Setter for [`NavMeshSettings::max_contour_simplification_error`]
    pub fn with_max_contour_simplification_error(
        mut self,
        max_contour_simplification_error: f32,
    ) -> Self {
        self.max_contour_simplification_error = max_contour_simplification_error;

        self
    }
    /// Setter for [`NavMeshSettings::max_edge_length`]
    pub fn with_max_edge_length(mut self, max_edge_length: u16) -> Self {
        self.max_edge_length = max_edge_length;

        self
    }

    /// TODO: this breaks when any meshes are concave like heightfields. do not use this yet.
    /// Setter for [`NavMeshSettings::experimental_detail_mesh_generation`]
    ///
    /// **Experimental**: This may have issues at the edges of regions.
    pub fn with_experimental_detail_mesh_generation(
        mut self,
        detail_mesh_generation_settings: DetailMeshSettings,
    ) -> Self {
        self.experimental_detail_mesh_generation = Some(detail_mesh_generation_settings);

        self
    }

    /// Returns the length of a tile's side in world units.
    #[inline]
    pub fn get_tile_size(&self) -> f32 {
        self.cell_width * f32::from(self.tile_width.get())
    }

    #[inline]
    pub fn get_border_size(&self) -> f32 {
        f32::from(self.walkable_radius) * self.cell_width
    }
    /// Returns the tile coordinate that contains the supplied ``world_position``.
    // TODO: this assumes position is always in contained, and only checking xz
    #[inline]
    pub fn get_tile_containing_position(
        &self,
        world_position: Vec2,
        transform: &GlobalTransform,
    ) -> UVec2 {
        let local = transform.affine().inverse().transform_point(Vec3::new(
            world_position.x,
            0.,
            world_position.y,
        ));

        let x = local.x + self.world_half_extents.x;
        let z = local.z + self.world_half_extents.z;

        // 3) Divide by tile_size, floor to get integer tile index, clamp ≥ 0
        let tile_size = self.get_tile_size();
        let tx = (x / tile_size).floor().max(0.0) as u32;
        let tz = (z / tile_size).floor().max(0.0) as u32;

        UVec2::new(tx, tz)

        // (local.xz() / self.get_tile_size()).as_uvec2()
    }

    /// Returns the minimum bound of a tile in local space.
    #[inline]
    pub fn get_tile_minimum_bound(&self) -> Vec3 {
        Vec3::new(
            -self.get_tile_size() / 2.0,
            -self.world_half_extents.y,
            -self.get_tile_size() / 2.0,
        )
    }

    /// Returns the origin of a tile on the XZ-plane including the border area.
    #[inline]
    pub fn get_tile_minimum_bound_with_border(&self) -> Vec3 {
        let boarder_size = self.get_border_size();
        self.get_tile_minimum_bound() - Vec3::new(boarder_size, 0.0, boarder_size)
    }

    #[inline]
    pub fn get_tile_side_with_border(&self) -> usize {
        usize::from(self.tile_width.get()) + usize::from(self.walkable_radius) * 2
    }

    #[inline]
    pub fn get_border_side(&self) -> usize {
        // Not technically useful currently but in case.
        self.walkable_radius.into()
    }
}

/// Managed list of agents in the waymap.
#[derive(Component, Default, Debug, Reflect)]
#[relationship_target(relationship = AgentNav)]
pub struct NavAgents(Vec<Entity>);

/// Managed list of characters in the waymap.
#[derive(Component, Default, Debug, Reflect)]
#[relationship_target(relationship = CharacterWaymap)]
pub struct NavCharacters(Vec<Entity>);

#[derive(Component, Default, Debug, Reflect)]
#[relationship_target(relationship = TileWaymap)]
pub struct NavTiles(Vec<Entity>);

/// Set of all tiles that need to be rebuilt.
#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct TileLookup(pub HashMap<UVec2, Entity>);

/// Set of all tiles that need to be rebuilt.
#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct DirtyTiles(pub HashSet<UVec2>);

/// List of tasks that are currently generating tiles.
#[derive(Component, Default, Deref, DerefMut)]
pub struct NavGenerationTasks(pub Vec<NavMeshGenerationJob>);

#[derive(Component, Clone, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct NavAabb(pub Aabb3d);

/// A task that is generating a nav-mesh tile.
pub struct NavMeshGenerationJob {
    pub entity: Entity,
    // pub generation: u64,
    pub task: Task<TileBuildResult>,
}

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component)]
pub struct AgentOptions {
    /// The distance that an agent will consider avoiding another agent
    #[inspector(min = 1.0, max = 50.0, speed = 1.0, display = NumberDisplay::Slider)]
    pub neighbourhood: f32,
    // The time into the future that collisions with other agents should be
    /// avoided.
    #[inspector(min = 0.5, max = 5.0, speed = 0.1, display = NumberDisplay::Slider)]
    pub avoidance_time_horizon: f32,
    /// The time into the future that collisions with obstacles should be
    /// avoided.
    #[inspector(min = 0.1, max = 5.0, speed = 0.1, display = NumberDisplay::Slider)]
    pub obstacle_avoidance_time_horizon: f32,
    /// The avoidance responsibility to use when an agent has reached its target.
    /// A value of 1.0 is the default avoidance responsibility. A value of 0.0
    /// would mean no avoidance responsibility, but a value of 0.0 is invalid and
    /// may panic. This should be a value between 0.0 and 1.0.
    #[inspector(min = 0.01, max = 1.0, speed = 0.01, display = NumberDisplay::Slider)]
    pub reached_destination_avoidance_responsibility: f32,
}

impl Default for AgentOptions {
    fn default() -> Self {
        Self {
            neighbourhood: 5.0,
            avoidance_time_horizon: 1.0,
            obstacle_avoidance_time_horizon: 0.5,
            reached_destination_avoidance_responsibility: 0.1,
        }
    }
}

#[derive(Clone, Reflect, Debug)]
pub struct DetailMeshSettings {
    /// The maximum acceptible error in height between the nav-mesh polygons & the true world (in cells).
    pub max_height_error: NonZeroU16,
    /// Determines how often (in cells) to sample the height when generating the height-corrected nav-mesh.
    ///
    /// This greatly affects generation performance. Higher values reduce samples by half to the previous one.
    /// Ex. 1.0, 0.5, 0.25, 0.125.
    ///
    /// **Suggested value:** >=2. Start high & reduce as needed.  
    pub sample_step: NonZeroU8,
}
