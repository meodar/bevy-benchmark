use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

impl EnemySpawnTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }

    pub fn timer_as_mut(&mut self) -> &mut Timer {
        &mut self.0
    }

    pub fn timer(&self) -> &Timer {
        &self.0
    }
}
