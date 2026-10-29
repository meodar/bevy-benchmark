use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Score,
    Quit,
}
