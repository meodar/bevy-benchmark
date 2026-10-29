use bevy::prelude::*;
use camera_mode::CameraMode;

pub mod camera_mode;

#[derive(Resource, Default)]
pub struct Game {
    camera_mode: CameraMode,
}

impl Game {
    pub fn set_camera_mode(&mut self, camera_mode: CameraMode) {
        self.camera_mode = camera_mode;
    }

    pub fn camera_mode(&self) -> &CameraMode {
        &self.camera_mode
    }
}
