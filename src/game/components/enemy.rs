use bevy::prelude::*;

pub mod enemy_bullet;
pub mod enemy_shooting_timer;
pub mod enemy_state;
pub mod side_enemy;
pub mod top_enemy;

#[derive(Component, Clone)]
pub struct Enemy;
