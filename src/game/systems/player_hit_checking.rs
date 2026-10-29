use bevy::prelude::*;

use crate::game::components::{
    enemy::enemy_bullet::Bullet, health::Health, player::Player,
    radial_hit_detector::RadialHitDetector,
};

pub fn player_hit_check(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity, &Bullet)>,
    mut player_query: Query<(&Transform, &RadialHitDetector, &mut Health), With<Player>>,
) {
    if let Ok((player_tf, player_rhd, mut player_hp)) = player_query.single_mut() {
        for (bullet_tf, bullet_entity, bullet) in bullet_query.iter() {
            if player_rhd.is_hit(&bullet_tf.translation, &player_tf.translation) {
                player_hp.make_damage(bullet.damage());
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
