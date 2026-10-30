use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, NetworkExt};
use crate::app_state::SystemStats;

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.system.refresh_disks();
        self.system.refresh_networks();
    }

    pub fn stats(&self) -> SystemStats {
        let cpu = self.system.global_processor_info().cpu_usage();
        let total_mem = self.system.total_memory() as f32;
        let used_mem = self.system.used_memory() as f32;
        let mem_percent = if total_mem > 0.0 { (used_mem / total_mem) * 100.0 } else { 0.0 };

        let mut total_disk: u64 = 0;
        let mut used_disk: u64 = 0;
        for disk in self.system.disks() {
            total_disk += disk.total_space();
            used_disk += disk.total_space() - disk.available_space();
        }
        let disk_percent = if total_disk > 0 {
            (used_disk as f32 / total_disk as f32) * 100.0
        } else {
            0.0
        };

        SystemStats {
            cpu,
            mem: mem_percent,
            disk: disk_percent,
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}