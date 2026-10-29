use bevy::prelude::*;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use sysinfo::{Pid, System};

use crate::game::resources::perf_monitor::PerfMonitor;

pub fn setup_perf_monitor(mut commands: Commands, time: Res<Time>) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let pid = Pid::from_u32(std::process::id());
    let nvml = nvml_wrapper::Nvml::init().expect("Failed to initialize NVML");
    // Готовим CSV-файл
    let filename = format!("perf_{}.csv", Local::now().format("%Y-%m-%d_%H-%M-%S"));
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&filename)
        .ok();

    // Запишем заголовок
    if let Some(f) = file.as_mut() {
        let _ = writeln!(
            f,
            "second,cpu_percent,ram_mib,gpu_percent,gpu_mem,fps"
        );
    }
    commands.insert_resource(PerfMonitor::new(sys, pid, time.elapsed_secs(), nvml, file));
}
