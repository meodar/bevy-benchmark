use bevy::prelude::*;

use crate::game::components::enemy::Enemy;

pub fn despawn_enemies_out_of_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.z >= 800.0 {
            commands.entity(entity).despawn();
        }
    }
}
