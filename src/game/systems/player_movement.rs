use bevy::prelude::*;

use crate::game::{
    components::{player::Player, speed::Speed},
    resources::game::{Game, camera_mode::CameraMode},
};

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform_query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
    game: Res<Game>,
) {
    if let Ok((mut player_transform, speed)) = transform_query.single_mut() {
        match game.camera_mode() {
            CameraMode::Top { .. } => {
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    player_transform.translation -=
                        Vec3::new(0.0, 0.0, speed.0 * time.delta_secs());
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    player_transform.translation +=
                        Vec3::new(0.0, 0.0, speed.0 * time.delta_secs());
                }
                if keyboard_input.pressed(KeyCode::ArrowLeft) {
                    player_transform.translation -=
                        Vec3::new(speed.0 * time.delta_secs(), 0.0, 0.0);
                }
                if keyboard_input.pressed(KeyCode::ArrowRight) {
                    player_transform.translation +=
                        Vec3::new(speed.0 * time.delta_secs(), 0.0, 0.0);
                }
            }
            CameraMode::Side { .. } => {
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    player_transform.translation +=
                        Vec3::new(0.0, speed.0 * time.delta_secs(), 0.0);
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    player_transform.translation -=
                        Vec3::new(0.0, speed.0 * time.delta_secs(), 0.0);
                }
                if keyboard_input.pressed(KeyCode::ArrowLeft) {
                    player_transform.translation +=
                        Vec3::new(0.0, 0.0, speed.0 * time.delta_secs());
                }
                if keyboard_input.pressed(KeyCode::ArrowRight) {
                    player_transform.translation -=
                        Vec3::new(0.0, 0.0, speed.0 * time.delta_secs());
                }
            }
        }
    }
}
