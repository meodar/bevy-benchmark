use bevy::prelude::*;

#[derive(Resource)]
pub struct BenchmarkData {
    pub timer: Timer,
    pub benchmark_value: u32,
    pub benchmark_sum: f32,
}

impl BenchmarkData {
    pub fn new(timer: Timer, benchmark_value: u32) -> Self {
        Self {
            timer,
            benchmark_value,
            benchmark_sum: 0.0,
        }
    }
}
