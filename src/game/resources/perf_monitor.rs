use bevy::prelude::*;
use nvml_wrapper::Nvml;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

pub mod tick_stats;

#[derive(Resource)]
pub struct PerfMonitor {
    system: System,
    pid: Pid,
    last_print: f32,
    nvml: Nvml,
    csv: Option<std::fs::File>,
    fps: u32,
    fps_frames: u32,
    fps_window_start: Option<f64>,
    tick_start: Option<Instant>,
    tick_time: Duration,
    ticks: u32,
    frames: u32,
}

impl PerfMonitor {
    pub fn new(
        system: System,
        pid: Pid,
        last_print: f32,
        nvml: Nvml,
        csv: Option<std::fs::File>,
    ) -> Self {
        Self {
            system,
            pid,
            last_print,
            nvml,
            csv,
            fps: 1,
            fps_frames: 0,
            fps_window_start: None,
            tick_start: None,
            tick_time: Duration::ZERO,
            ticks: 0,
            frames: 0,
        }
    }

    pub fn begin_tick(&mut self) {
        self.tick_start = Some(Instant::now());
    }

    pub fn end_tick(&mut self) {
        if let Some(start) = self.tick_start.take() {
            self.tick_time += start.elapsed();
            self.ticks += 1;
        }
    }

    pub fn count_frame(&mut self, real_secs: f64) {
        let window_start = *self.fps_window_start.get_or_insert(real_secs);
        self.frames += 1;
        self.fps_frames += 1;
        if real_secs - window_start >= 1.0 {
            self.fps = self.fps_frames;
            self.fps_frames = 0;
            self.fps_window_start = Some(window_start + (real_secs - window_start).floor());
        }
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn system_mut(&mut self) -> &mut System {
        &mut self.system
    }

    pub fn last_print(&self) -> f32 {
        self.last_print
    }

    pub fn set_last_print(&mut self, last_print: f32) {
        self.last_print = last_print;
    }

    pub fn nvml(&self) -> &Nvml {
        &self.nvml
    }

    pub fn csv_mut(&mut self) -> Option<&mut std::fs::File> {
        self.csv.as_mut()
    }
}
