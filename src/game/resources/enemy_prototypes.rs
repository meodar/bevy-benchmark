use bevy::prelude::*;

use crate::game::bundles::{side_enemy_bundle::SideEnemyBundle, top_enemy_bundle::TopEnemyBundle};

// ресурс с прототипами
#[derive(Resource, Clone)]
pub struct EnemyPrototypes {
    pub top: TopEnemyBundle,
    pub side: SideEnemyBundle,
}
