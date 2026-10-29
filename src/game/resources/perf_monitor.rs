use bevy::prelude::*;
use nvml_wrapper::Nvml;
use sysinfo::{Pid, System};

#[derive(Resource)]
pub struct PerfMonitor {
    system: System,
    pid: Pid,
    last_print: f32,
    nvml: Nvml,
    csv: Option<std::fs::File>,
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
        }
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
