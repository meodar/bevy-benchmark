use bevy::prelude::*;

use crate::game::components::terrain::Terrain;

pub fn move_terrain(mut transform_query: Query<&mut Transform, With<Terrain>>, time: Res<Time>) {
    for mut terrain_transform in transform_query.iter_mut() {
        terrain_transform.translation.z += 80.0 * time.delta_secs();
        if terrain_transform.translation.z >= 32000.0 {
            terrain_transform.translation.z -= 64000.0;
        }
    }
}
