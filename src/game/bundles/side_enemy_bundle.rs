use bevy::prelude::*;

use crate::{
    game::{
        bundles::enemy_base_bundle::EnemyBaseBundle,
        components::{
            enemy::{enemy_state::WorkloadCursor, side_enemy::SideEnemy},
            enemy_movement::{EnemyMovement, MovementMod},
        },
    },
    random_table,
};

#[derive(Bundle, Clone)]
pub struct SideEnemyBundle {
    pub base: EnemyBaseBundle,
    pub marker: SideEnemy,
    pub movement: EnemyMovement,
    pub movement_mod: MovementMod,
    pub scene: WorldAssetRoot,
}

impl SideEnemyBundle {
    pub fn prototype(assets: Handle<WorldAsset>) -> Self {
        Self {
            base: EnemyBaseBundle::new(7.69194, Vec3::new(0.0, 0.0, 12.5288)),
            marker: SideEnemy,
            movement: EnemyMovement::Sin,
            movement_mod: MovementMod::zero(),
            scene: WorldAssetRoot(assets),
        }
    }
    pub fn spawned_at(mut self, t: Vec3) -> Self {
        let spawn_index = random_table::next_spawn_index();
        self.base.transform.translation = t;
        self.base.workload_cursor = WorkloadCursor(spawn_index);
        self.movement = EnemyMovement::from_table(spawn_index);
        self.movement_mod = MovementMod::from_table(spawn_index);
        self
    }
}
