use std::time::Duration;

use bevy::prelude::*;
use resources::game::Game;
use systems::{
    change_camera, check_player_health, enemy_hit_check, enemy_shoot, enemy_spawn, move_bullet,
    move_enemy_bullet, move_player, move_side_enemy, move_terrain, move_top_enemy,
    player_hit_check, player_shoot, process_skybox, setup_game,
};

use crate::{
    game::systems::{
        benchmark_heavy_tick, benchmark_spawn, despawn_bullets_out_of_bounds,
        despawn_enemies_out_of_bounds, despawn_enemy_bullets_out_of_bounds, enemy_benchmark,
        print_metrics, setup_perf_monitor, timer_benchmark_spawn,
    },
    game_state::GameState,
};

mod bundles;
mod components;
mod resources;
mod systems;

/// Частота физики Godot (physics/common/physics_ticks_per_second)
const PHYSICS_TICKS_PER_SECOND: f64 = 60.0;
/// Godot (physics/common/max_physics_steps_per_frame): если кадр дольше, игра замедляется
const MAX_PHYSICS_STEPS_PER_FRAME: f64 = 8.0;

pub fn game_plugin(app: &mut App) {
    // То, что в Godot-версиях живёт в _physics_process, здесь идёт в FixedUpdate
    // с той же частотой и тем же ограничением шагов за кадр.
    app.insert_resource(Time::<Fixed>::from_hz(PHYSICS_TICKS_PER_SECOND));
    app.world_mut()
        .resource_mut::<Time<Virtual>>()
        .set_max_delta(Duration::from_secs_f64(
            MAX_PHYSICS_STEPS_PER_FRAME / PHYSICS_TICKS_PER_SECOND,
        ));

    app.add_systems(
        OnEnter(GameState::Game),
        (setup_game, /*setup_hud, */ setup_perf_monitor),
    )
    .init_resource::<Game>()
    // Ввод и таймеры (в Godot — Timer в режиме Idle) — раз в кадр
    .add_systems(
        Update,
        (
            player_shoot,
            change_camera,
            enemy_spawn,
            enemy_shoot,
            process_skybox,
            timer_benchmark_spawn,
            benchmark_spawn,
            print_metrics,
        )
            .run_if(in_state(GameState::Game)),
    )
    .add_systems(
        FixedUpdate,
        (
            move_player,
            move_bullet,
            move_side_enemy,
            move_top_enemy,
            enemy_benchmark,
            benchmark_heavy_tick,
            enemy_hit_check,
            move_enemy_bullet,
            player_hit_check,
            check_player_health,
            move_terrain,
            despawn_enemies_out_of_bounds,
        )
            .run_if(in_state(GameState::Game)),
    )
    .add_systems(
        FixedUpdate,
        (
            despawn_bullets_out_of_bounds,
            despawn_enemy_bullets_out_of_bounds,
        )
            .run_if(in_state(GameState::Game)),
    );
}
