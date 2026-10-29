use bevy::prelude::*;
use cleanup::cleanup_menu;
use components::main_menu_screen::MainMenuScreen;
use setup::setup_menu;
use state::MenuState;
use systems::{
    button_action::button_action, menu_action::menu_action, screen_despawner::despawn_screen,
};

use crate::game_state::GameState;

mod cleanup;
mod components;
mod main_menu;
mod setup;
mod state;
mod systems;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(MenuState::Main), main_menu::setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<MainMenuScreen>)
        .add_systems(Update, button_action)
        .add_observer(menu_action);
}
 