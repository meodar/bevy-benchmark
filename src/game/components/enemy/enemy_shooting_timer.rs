use core::time::Duration;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct EnemyShootingTimer {
    timer: Timer,
}

impl EnemyShootingTimer {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }

    pub fn can_shoot(&mut self, duration: Duration) -> bool {
        self.timer.tick(duration);
        self.timer.is_finished()
    }
}
