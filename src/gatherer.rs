use super::structs::SystemInfo;
use super::utils::bytes_to_mb;
use std::{time::SystemTime, cell::RefMut};
use sysinfo::{CpuExt, System, SystemExt};

pub fn create_sys() -> System {
    let mut sys = System::new_all();

    sys.refresh_all();
    return sys;
}

pub fn sys_info_gatherer(sys: RefMut<System>) -> SystemInfo {
    SystemInfo {
        time: SystemTime::now(),
        used_memory: bytes_to_mb(sys.used_memory()),
        free_memory: bytes_to_mb(sys.free_memory()),
        used_swap: bytes_to_mb(sys.used_swap()),
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        cpu_freq: sys.global_cpu_info().frequency(),
    }
}
