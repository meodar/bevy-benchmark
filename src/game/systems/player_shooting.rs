use bevy::prelude::*;

use crate::game::components::{
    bullet::Bullet, player::Player, shooting_point::ShootingPoint, velocity::Velocity,
};

pub fn player_shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut transform_query: Query<(&mut Transform, &ShootingPoint), With<Player>>,
    player_query: Query<&Player>,
) {
    if let Ok(_) = player_query.single() {
        if let Ok((player_transform, shooting_point)) = transform_query.single_mut() {
            if keyboard_input.just_pressed(KeyCode::Space) {
                let direction = player_transform.rotation.mul_vec3(Vec3::Z).normalize();

                // Спавн пули
                let bullet_transform = shooting_point.bullet_transform(&player_transform);
                commands.spawn((
                    WorldAssetRoot(
                        asset_server.load(
                            GltfAssetLabel::Scene(0).from_asset("models\\bullet\\bullet.gltf"),
                        ),
                    ),
                    bullet_transform,
                    Bullet,
                    Velocity(direction * 1000.0),
                ));
            }
        }
    }
}
