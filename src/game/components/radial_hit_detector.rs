use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct RadialHitDetector {
    radius: f32,
}

impl RadialHitDetector {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_hit(&self, first_obj_pos: &Vec3, second_obj_pos: &Vec3) -> bool {
        first_obj_pos.distance_squared(*second_obj_pos) < self.radius * self.radius
    }
}
