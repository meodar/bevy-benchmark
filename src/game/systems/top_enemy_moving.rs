use bevy::prelude::*;

use crate::game::components::{
    enemy::{enemy_state::TimeElapsed, top_enemy::TopEnemy},
    enemy_movement::{EnemyMovement, MovementMod},
    speed::Speed,
};

pub fn move_top_enemy(
    mut transform_query: Query<
        (
            &mut Transform,
            &mut TimeElapsed,
            &Speed,
            &EnemyMovement,
            &MovementMod,
        ),
        With<TopEnemy>,
    >,
    time: Res<Time>,
) {
    for (mut enemy_transform, mut time_elapsed, enemy_speed, movement, mods) in
        transform_query.iter_mut()
    {
        // Фаза траектории считается от времени жизни врага, как в Godot-версиях
        time_elapsed.0 += time.delta_secs_f64();
        let t = time_elapsed.0 as f32;
        let s = enemy_speed.0 * time.delta_secs();
        match movement {
            EnemyMovement::Lin => {
                enemy_transform.translation.z += s;
            }
            EnemyMovement::Cos => {
                enemy_transform.translation.x = mods.a * (mods.b * t + mods.c).cos() * 500.0;
                enemy_transform.translation.z += s;
            }
            EnemyMovement::Sin => {
                enemy_transform.translation.x = mods.a * (mods.b * t + mods.c).sin() * 500.0;
                enemy_transform.translation.z += s;
            }
            EnemyMovement::Log => {
                let z = enemy_transform.translation.z + s;
                enemy_transform.translation.x = mods.a * (z.abs().max(1.0) + mods.c).ln() * 200.0;
                enemy_transform.translation.z = z;
            }
            EnemyMovement::Sqrt => {
                let z = enemy_transform.translation.z + s;
                enemy_transform.translation.x = mods.a * (z.abs().sqrt() + mods.c) * 100.0;
                enemy_transform.translation.z = z;
            }
        }
    }
}
