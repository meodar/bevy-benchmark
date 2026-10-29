use bevy::prelude::*;

#[derive(Resource)]
pub struct SkyboxImage {
    handle: Handle<Image>,
    is_loaded: bool,
}

impl SkyboxImage {
    pub fn new(handle: Handle<Image>, is_loaded: bool) -> Self {
        Self { handle, is_loaded }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn handle(&self) -> &Handle<Image> {
        &self.handle
    }

    pub fn loaded(&mut self) {
        self.is_loaded = true;
    }
}
