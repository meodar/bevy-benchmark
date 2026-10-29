use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    current: i32,
}

impl Health {
    pub fn new(value: i32) -> Self {
        Self { current: value }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn make_damage(&mut self, damage: i32) -> bool {
        self.current = self.current - damage;
        self.current <= 0
    }
}
