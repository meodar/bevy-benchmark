use bevy::prelude::*;

use crate::{game::resources::enemy_prototypes::EnemyPrototypes, random_table};

/// Позиции чтения в таблицах случайных чисел — как поля EnemiesManager в Godot-версиях
#[derive(Resource, Default)]
pub struct EnemiesManager {
    spawn_cursor: usize,
    side_cursor: usize,
    tick_cursor: usize,
    workload_cursor: usize,
}

impl EnemiesManager {
    pub fn spawn_enemy(&mut self, commands: &mut Commands, protos: &EnemyPrototypes, is_top: bool) {
        let table = random_table::table();
        if is_top {
            let x = table.spawn_x(self.spawn_cursor);
            commands.spawn(protos.top.clone().spawned_at(Vec3::new(x, 0.0, -800.0)));
        } else {
            let y = table.spawn_y(self.spawn_cursor);
            commands.spawn(protos.side.clone().spawned_at(Vec3::new(0.0, y, -800.0)));
        }
        self.spawn_cursor = self.spawn_cursor.wrapping_add(1);
    }

    pub fn next_is_top(&mut self) -> bool {
        let is_top = random_table::table().spawn_top(self.side_cursor);
        self.side_cursor = self.side_cursor.wrapping_add(1);
        is_top
    }

    pub fn next_heavy_tick(&mut self) -> bool {
        let heavy_tick = random_table::table().heavy_tick(self.tick_cursor);
        self.tick_cursor = self.tick_cursor.wrapping_add(1);
        heavy_tick
    }

    pub fn workload_cursor_mut(&mut self) -> &mut usize {
        &mut self.workload_cursor
    }
}
