use bevy::prelude::*;

use crate::{
    game::resources::{benchmark_timer::BenchmarkData, enemies_manager::EnemiesManager},
    random_table,
};

/// Редкий «тяжёлый» тик менеджера врагов: в Godot-версиях это его _physics_process
pub fn benchmark_heavy_tick(
    mut benchmark_data: ResMut<BenchmarkData>,
    mut manager: ResMut<EnemiesManager>,
) {
    if manager.next_heavy_tick() {
        let table = random_table::table();
        let mut acc = 0.0_f32;
        let mut local_data = Vec::new();
        let cursor = manager.workload_cursor_mut();
        for _ in 0..500 {
            let i = table.workload_500(*cursor);
            *cursor = cursor.wrapping_add(1);
            let x = (i as f32 * std::f32::consts::PI / 180.0).sin() * ((i as f32 + 1.0).sqrt());
            acc += x.powf(1.5);
            local_data.push(acc);
        }

        benchmark_data.benchmark_sum = acc;
    }
}
