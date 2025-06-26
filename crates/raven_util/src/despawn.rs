use bevy::prelude::*;

pub fn despawn_by_y<const Y: i32>(mut commands: Commands, query: Query<(Entity, &Transform)>) {
    for (entity, transform) in &query {
        if transform.translation.y < Y as f32 {
            commands.entity(entity).despawn();
        }
    }
}