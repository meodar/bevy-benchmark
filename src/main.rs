use std::path::PathBuf;

use bevy::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, window::PresentMode};
use game::game_plugin;
use game_state::GameState;
use menu::menu_plugin;

mod game;
mod game_state;
mod menu;
mod random_table;
mod utils;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    let mut app = App::new();

    let default_plugin = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::Fifo,
            ..default()
        }),
        ..default()
    });

    let default_plugin = if cfg!(feature = "lldb_debug") {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let assets_dir = manifest_dir.join("assets").to_string_lossy().to_string();
        default_plugin.set(AssetPlugin {
            file_path: assets_dir,
            ..default()
        })
    } else {
        default_plugin
    };

    app.add_plugins(default_plugin)
        .init_state::<GameState>()
        .add_plugins((menu_plugin, game_plugin))
        .add_plugins(FrameTimeDiagnosticsPlugin::default());

    #[cfg(feature = "fps_debug")]
    {
        use utils::fps_printer::print_fps_system;
        app.add_systems(Update, print_fps_system);
    }

    app.run();
}
