use bevy::prelude::*;

use crate::game::components::bullet::Bullet;

pub fn despawn_bullets_out_of_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.z.abs() > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}
