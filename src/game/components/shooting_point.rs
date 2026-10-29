use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct ShootingPoint(pub Vec3);

impl ShootingPoint {
    pub fn bullet_transform(&self, shooter: &Transform) -> Transform {
        shooter.with_translation(shooter.transform_point(self.0))
    }
}
