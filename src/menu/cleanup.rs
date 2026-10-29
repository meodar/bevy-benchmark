use bevy::prelude::*;

use super::components::menu_camera::MenuCamera;

pub fn cleanup_menu(mut commands: Commands, camera_entity: Single<Entity, With<MenuCamera>>) {
    commands.entity(*camera_entity).despawn();
}
