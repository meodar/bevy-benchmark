use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    damage: i32,
}

impl Bullet {
    pub fn new(damage: i32) -> Self {
        Self { damage }
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}
