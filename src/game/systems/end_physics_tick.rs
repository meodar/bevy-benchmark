use bevy::ecs::system::ResMut;

use crate::game::resources::perf_monitor::PerfMonitor;

pub fn end_physics_tick(perf: Option<ResMut<PerfMonitor>>) {
    if let Some(mut perf) = perf {
        perf.end_tick();
    }
}
