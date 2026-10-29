use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use crate::{
    game_state::GameState,
    menu::{components::menu_button_action::MenuButtonAction, state::MenuState},
};

pub fn menu_action(
    activate: On<Activate>,
    action_query: Query<&MenuButtonAction>,
    mut app_exit_events: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok(menu_button_action) = action_query.get(activate.entity) else {
        return;
    };
    match menu_button_action {
        MenuButtonAction::Play => {
            game_state.set(GameState::Game);
            menu_state.set(MenuState::Disabled);
        }
        MenuButtonAction::Settings => {
            menu_state.set(MenuState::Settings);
        }
        MenuButtonAction::Score => {
            menu_state.set(MenuState::Score);
        }
        MenuButtonAction::Quit => {
            app_exit_events.write(AppExit::Success);
        }
    }
}
