use bevy::prelude::*;

use crate::game::{
    components::{
        enemy::{enemy_bullet::Bullet, enemy_shooting_timer::EnemyShootingTimer, Enemy},
        shooting_point::ShootingPoint,
        velocity::Velocity,
    },
    resources::bullet_asset::BulletAssets,
};

pub fn enemy_shoot(
    mut commands: Commands,
    time: Res<Time>,
    bullet_asset: Res<BulletAssets>,
    mut enemy_data_query: Query<
        (&mut EnemyShootingTimer, &Transform, &ShootingPoint),
        With<Enemy>,
    >,
) {
    for (mut shooting_timer, transform, shooting_point) in enemy_data_query.iter_mut() {
        if shooting_timer.can_shoot(time.delta()) {
            let direction = transform.rotation.mul_vec3(Vec3::Z).normalize();

            // Спавн пули
            let bullet_transform = shooting_point.bullet_transform(transform);
            commands.spawn((
                WorldAssetRoot(bullet_asset.bullet_scene().clone()),
                bullet_transform,
                Bullet::new(300),
                Velocity(direction * 1000.0),
            ));
        }
    }
}
