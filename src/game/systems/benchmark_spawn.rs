use bevy::prelude::*;

use crate::game::resources::{enemies_manager::EnemiesManager, enemy_prototypes::EnemyPrototypes};

pub fn benchmark_spawn(
    mut commands: Commands,
    mut manager: ResMut<EnemiesManager>,
    protos: Res<EnemyPrototypes>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        // 50 top enemies
        for _ in 0..50 {
            manager.spawn_enemy(&mut commands, &protos, true);
        }
        // 50 side enemies
        for _ in 0..50 {
            manager.spawn_enemy(&mut commands, &protos, false);
        }
    }
}
