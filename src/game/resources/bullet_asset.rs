use bevy::prelude::*;

#[derive(Resource)]
pub struct BulletAssets {
    bullet_scene: Handle<WorldAsset>,
}

impl BulletAssets {
    pub fn new(bullet_scene: Handle<WorldAsset>) -> Self {
        Self { bullet_scene }
    }

    pub fn bullet_scene(&self) -> &Handle<WorldAsset> {
        &self.bullet_scene
    }
}
