use bevy::prelude::*;

use crate::game::resources::{
    enemies_manager::EnemiesManager, enemy_prototypes::EnemyPrototypes,
    enemy_spawn_timer::EnemySpawnTimer,
};

pub fn enemy_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut manager: ResMut<EnemiesManager>,
    protos: Res<EnemyPrototypes>,
) {
    enemy_spawn_timer.timer_as_mut().tick(time.delta());

    if enemy_spawn_timer.timer().is_finished() {
        let is_top = manager.next_is_top();
        manager.spawn_enemy(&mut commands, &protos, is_top);
    }
}
