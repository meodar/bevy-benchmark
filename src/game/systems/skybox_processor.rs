use bevy::prelude::*;
use bevy::{
    light::Skybox,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use crate::game::resources::skybox_image::SkyboxImage;

pub fn process_skybox(
    mut images: ResMut<Assets<Image>>,
    mut skybox_image: ResMut<SkyboxImage>,
    asset_server: Res<AssetServer>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !skybox_image.is_loaded() && asset_server.load_state(skybox_image.handle()).is_loaded() {
        if let Some(mut image) = images.get_mut(skybox_image.handle()) {
            let w = image.texture_descriptor.size.width;
            let h = image.texture_descriptor.size.height;
            println!("{} {}", w, h);
            if w * 6 == h && image.reinterpret_stacked_2d_as_array(6).is_ok() {
                image.texture_view_descriptor = Some(TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::Cube),
                    ..default()
                });
            }
 
            if let Ok(mut skybox) = skyboxes.single_mut() {
                skybox.image = Some(skybox_image.handle().clone());
            }

            skybox_image.loaded();
        }
    }
}
