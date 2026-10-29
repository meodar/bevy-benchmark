use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use crate::game::{
    components::player::Player,
    resources::game::{Game, camera_mode::CameraMode},
};

pub fn change_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_transform_query: Query<&mut Transform, With<Camera3d>>,
    mut player_tf_query: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
    mut game: ResMut<Game>,
) {
    if let Ok(mut player_tf) = player_tf_query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::ControlLeft) {
            match game.camera_mode().clone() {
                CameraMode::Top { last_side_pos } => {
                    camera_transform_query.iter_mut().for_each(|mut camera_tf| {
                        camera_tf.rotate_local(Quat::from_rotation_z(FRAC_PI_2));
                        camera_tf.rotate_local(Quat::from_rotation_x(FRAC_PI_2));
                        camera_tf.translation += Vec3::new(1000.0, -1000.0, 0.0);
                    });
                    game.set_camera_mode(CameraMode::Side {
                        last_top_pos: player_tf.translation.x,
                    });
                    player_tf.translation = Vec3::new(0.0, last_side_pos, player_tf.translation.z);
                }
                CameraMode::Side { last_top_pos } => {
                    camera_transform_query.iter_mut().for_each(|mut camera_tf| {
                        camera_tf.rotate_local(Quat::from_rotation_z(-FRAC_PI_2));
                        camera_tf.rotate_local(Quat::from_rotation_y(-FRAC_PI_2));
                        camera_tf.translation += Vec3::new(-1000.0, 1000.0, 0.0);
                    });
                    game.set_camera_mode(CameraMode::Top {
                        last_side_pos: player_tf.translation.y,
                    });
                    player_tf.translation = Vec3::new(last_top_pos, 0.0, player_tf.translation.z);
                }
            }
        }
    }
}
