use bevy::prelude::*;

use crate::random_table;

#[derive(Component, Clone)]
pub enum EnemyMovement {
    Lin,
    Cos,
    Sin,
    Log,
    Sqrt,
}

impl EnemyMovement {
    /// Вариант движения врага с порядковым номером `spawn_index` из общей таблицы
    pub fn from_table(spawn_index: usize) -> Self {
        match random_table::table().movement_kind(spawn_index) {
            0 => EnemyMovement::Lin,
            1 => EnemyMovement::Cos,
            2 => EnemyMovement::Sin,
            3 => EnemyMovement::Log,
            _ => EnemyMovement::Sqrt,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct MovementMod {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl MovementMod {
    pub fn zero() -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }

    pub fn from_table(spawn_index: usize) -> Self {
        let table = random_table::table();
        Self {
            a: table.movement_mod(spawn_index * 3),
            b: table.movement_mod(spawn_index * 3 + 1),
            c: table.movement_mod(spawn_index * 3 + 2),
        }
    }
}
