use bevy::prelude::*;

use crate::game::resources::{
    benchmark_timer::BenchmarkData, enemies_manager::EnemiesManager,
    enemy_prototypes::EnemyPrototypes,
};

pub fn timer_benchmark_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut benchmark_data: ResMut<BenchmarkData>,
    mut manager: ResMut<EnemiesManager>,
    protos: Res<EnemyPrototypes>,
) {
    benchmark_data.timer.tick(time.delta());

    if benchmark_data.timer.is_finished() {
        for _ in 0..benchmark_data.benchmark_value {
            manager.spawn_enemy(&mut commands, &protos, true);
        }
        for _ in 0..benchmark_data.benchmark_value {
            manager.spawn_enemy(&mut commands, &protos, false);
        }
        benchmark_data.benchmark_value += 10
    }
}
