use bevy::prelude::*;

use super::{components::menu_camera::MenuCamera, state::MenuState};

pub fn setup_menu(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    commands.spawn((Camera2d, MenuCamera));
    menu_state.set(MenuState::Main);
}
