use bevy::prelude::*;

use crate::game::components::{bullet::Bullet, velocity::Velocity};

pub fn move_bullet(
    mut transform_query: Query<(&Velocity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in transform_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}
