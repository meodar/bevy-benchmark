use bevy::prelude::*;

use crate::game::components::{
    enemy::{
        Enemy,
        enemy_shooting_timer::EnemyShootingTimer,
        enemy_state::{TimeElapsed, WorkloadCursor},
    },
    radial_hit_detector::RadialHitDetector,
    shooting_point::ShootingPoint,
    speed::Speed,
};

/// Радиус пули игрока: сфера 1.0028 в масштабе игрока (20)
const PLAYER_BULLET_RADIUS: f32 = 1.0028 * 20.0;

#[derive(Bundle, Clone)]
pub struct EnemyBaseBundle {
    pub transform: Transform,
    pub enemy: Enemy,
    pub speed: Speed,
    pub hit: RadialHitDetector,
    pub shooting: EnemyShootingTimer,
    pub shooting_point: ShootingPoint,
    pub time_elapsed: TimeElapsed,
    pub workload_cursor: WorkloadCursor,
}

impl EnemyBaseBundle {
    /// Радиус коллизии и точка вылета пули — в координатах модели, как в Godot-сцене
    pub fn new(body_radius: f32, shooting_point: Vec3) -> Self {
        Self {
            transform: Transform::from_scale(Vec3::splat(10.0))
                .with_translation(Vec3::new(0.0, 0.0, -800.0)),
            enemy: Enemy,
            speed: Speed(200.0),
            hit: RadialHitDetector::new(body_radius * 10.0 + PLAYER_BULLET_RADIUS),
            shooting: EnemyShootingTimer::new(2.0),
            shooting_point: ShootingPoint(shooting_point),
            time_elapsed: TimeElapsed::default(),
            workload_cursor: WorkloadCursor::default(),
        }
    }
}
