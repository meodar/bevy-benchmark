use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use std::io::Write;
use sysinfo::ProcessesToUpdate;

use crate::game::resources::perf_monitor::PerfMonitor;

pub fn print_metrics(
    mut perf: ResMut<PerfMonitor>,
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if time.elapsed_secs() - perf.last_print() > 0.25 {
        // CPU/RAM (процесс)
        perf.system_mut()
            .refresh_processes(ProcessesToUpdate::All, true);
        perf.system_mut().refresh_memory();

        let mut cpu_percent = 0.0f32;
        let mut ram_mib = 0.0f32;

        if let Some(process) = perf.system().process(perf.pid()) {
            cpu_percent = process.cpu_usage();
            // process.memory() в KiB -> переведём в MiB
            ram_mib = process.memory() as f32 / 1024.0 / 1024.0;
            println!("CPU usage by process: {:.2}%", cpu_percent);
            println!("RAM usage by process: {:.2} MiB", ram_mib);
        }

        // GPU per-process (NVML), суммируем по всем устройствам
        let mut gpu_percent_sum: u32 = 0;
        let mut gpu_mem_mib_sum: u32 = 0;

        if let Ok(count) = perf.nvml().device_count() {
            for i in 0..count {
                if let Ok(dev) = perf.nvml().device_by_index(i) {
                    if let Ok(stats) = dev.process_utilization_stats(None) {
                        if let Some(sample) = stats.iter().find(|s| s.pid == perf.pid().as_u32()) {
                            // sm_util — загрузка GPU в %, mem_util — используемая память в MiB
                            gpu_percent_sum = gpu_percent_sum.saturating_add(sample.sm_util);
                            gpu_mem_mib_sum = gpu_mem_mib_sum.saturating_add(sample.mem_util);
                            println!(
                                "GPU {} usage (this process): {}% | Memory: {} MiB",
                                dev.name().unwrap_or_default(),
                                sample.sm_util,
                                sample.mem_util
                            );
                        }
                    }
                }
            }
        }

        // FPS (сглаженное)
        let mut fps_out: f32 = 0.0;
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(avg) = fps.smoothed() {
                fps_out = avg as f32;
                println!("FPS: {:.1}", avg);
            }
        }

        // Пишем строку в CSV: second, cpu%, ram MiB, gpu% sum, gpu_mem MiB sum, fps
        if let Some(file) = perf.csv_mut() {
            let _ = writeln!(
                file,
                "{},{:.2},{:.2},{},{},{}",
                time.elapsed_secs(),
                cpu_percent,
                ram_mib,
                gpu_percent_sum,
                gpu_mem_mib_sum,
                fps_out
            );
        }

        perf.set_last_print(time.elapsed_secs());
    }
}
