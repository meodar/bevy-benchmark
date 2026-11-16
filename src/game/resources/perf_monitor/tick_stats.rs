use std::{
    sync::atomic::{AtomicI64, AtomicU64, Ordering},
    time::Duration,
};

use crate::game::resources::perf_monitor::PerfMonitor;

#[cfg(windows)]
unsafe extern "system" {
    fn GetCurrentProcessorNumber() -> u32;
}

fn current_core() -> i64 {
    #[cfg(windows)]
    {
        i64::from(unsafe { GetCurrentProcessorNumber() })
    }
    #[cfg(not(windows))]
    {
        -1
    }
}

static WORKLOAD_NANOS: AtomicU64 = AtomicU64::new(0);
static WORKLOAD_CORE: AtomicI64 = AtomicI64::new(-1);

pub fn record_workload(elapsed: Duration) {
    WORKLOAD_NANOS.fetch_add(elapsed.as_nanos() as u64, Ordering::Relaxed);
    WORKLOAD_CORE.store(current_core(), Ordering::Relaxed);
}

pub struct TickStats {
    physics_ms: f64,
    steps_per_frame: f64,
    workload_ms: f64,
    core: i64,
}

impl TickStats {
    pub fn take_tick_stats(perf_monitor: &mut PerfMonitor) -> TickStats {
        let workload = Duration::from_nanos(WORKLOAD_NANOS.swap(0, Ordering::Relaxed));
        let per_tick = |total: Duration| {
            if perf_monitor.ticks > 0 {
                total.as_secs_f64() * 1000.0 / f64::from(perf_monitor.ticks)
            } else {
                0.0
            }
        };
        let stats = TickStats {
            physics_ms: per_tick(perf_monitor.tick_time),
            steps_per_frame: if perf_monitor.frames > 0 {
                f64::from(perf_monitor.ticks) / f64::from(perf_monitor.frames)
            } else {
                0.0
            },
            workload_ms: per_tick(workload),
            core: WORKLOAD_CORE.load(Ordering::Relaxed),
        };
        perf_monitor.tick_time = Duration::ZERO;
        perf_monitor.ticks = 0;
        perf_monitor.frames = 0;
        stats
    }

    pub fn physics_ms(&self) -> f64 {
        self.physics_ms
    }

    pub fn steps_per_frame(&self) -> f64 {
        self.steps_per_frame
    }

    pub fn workload_ms(&self) -> f64 {
        self.workload_ms
    }

    pub fn core(&self) -> i64 {
        self.core
    }
}
