use bevy::prelude::*;
use bevy::{picking::hover::Hovered, ui::Pressed, ui_widgets::Button};

use crate::menu::{
    components::selected_option::SelectedOption, HOVERED_BUTTON, HOVERED_PRESSED_BUTTON,
    NORMAL_BUTTON, PRESSED_BUTTON,
};

pub fn button_action(
    mut interaction_query: Query<
        (Has<Pressed>, &Hovered, &mut BackgroundColor, Option<&SelectedOption>),
        With<Button>,
    >,
) {
    for (pressed, hovered, mut background_color, selected) in &mut interaction_query {
        background_color.set_if_neq(match (pressed, hovered.get(), selected) {
            (true, _, _) | (false, false, Some(_)) => PRESSED_BUTTON.into(),
            (false, true, None) => HOVERED_PRESSED_BUTTON.into(),
            (false, true, Some(_)) => HOVERED_BUTTON.into(),
            (false, false, None) => NORMAL_BUTTON.into(),
        });
    }
}
