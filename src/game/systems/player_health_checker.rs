use bevy::prelude::*;

use crate::game::components::{health::Health, player::Player};

pub fn check_player_health(
    mut commands: Commands,
    player_data: Query<(&Health, Entity), With<Player>>,
) {
    if let Ok(player_data) = player_data.single() {
        let (player_health, player_entity) = player_data;
        if player_health.current() <= 0 {
            commands.entity(player_entity).despawn();
        }
    }
}
