use bevy::prelude::*;

use crate::game::components::{
    bullet::Bullet, enemy::Enemy, radial_hit_detector::RadialHitDetector,
};

pub fn enemy_hit_check(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    enemy_query: Query<(&Transform, &RadialHitDetector, Entity), With<Enemy>>,
) {
    for (bullet_tf, bullet_entity) in bullet_query.iter() {
        for (enemy_tf, radial_hit_detector, enemy_entity) in enemy_query.iter() {
            if radial_hit_detector.is_hit(&bullet_tf.translation, &enemy_tf.translation) {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
