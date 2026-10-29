use bevy::prelude::*;
use bevy::{
    color::palettes::css::LIGHT_GREEN,
    picking::hover::Hovered,
    ui_widgets::{ActivateOnPress, Button},
};

use crate::TEXT_COLOR;

use super::{
    NORMAL_BUTTON,
    components::{main_menu_screen::MainMenuScreen, menu_button_action::MenuButtonAction},
};

pub fn setup(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: FontSize::Px(33.0),
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(LIGHT_GREEN.into()),
                ))
                .with_children(|parent| {
                    // Построение элементов внутри панели меню

                    // Название
                    parent.spawn((
                        Text::new("Top Former"),
                        TextFont {
                            font_size: FontSize::Px(67.0),
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    // Кнопка "Играть"
                    parent
                        .spawn((
                            Button,
                            ActivateOnPress,
                            Hovered::default(),
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Play"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });

                    // Кнопка "Настройки"
                    parent
                        .spawn((
                            Button,
                            ActivateOnPress,
                            Hovered::default(),
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });

                    // Кнопка "Рекорд"
                    parent
                        .spawn((
                            Button,
                            ActivateOnPress,
                            Hovered::default(),
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Score,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Score"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });

                    // Кнопка "Выход"
                    parent
                        .spawn((
                            Button,
                            ActivateOnPress,
                            Hovered::default(),
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}
