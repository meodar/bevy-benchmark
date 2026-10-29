use bevy::prelude::*;

use crate::game::components::{enemy::enemy_bullet::Bullet, velocity::Velocity};

pub fn move_enemy_bullet(
    mut transform_query: Query<(&Velocity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in transform_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}
