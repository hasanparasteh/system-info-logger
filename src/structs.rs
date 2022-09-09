use serde::{Deserialize, Serialize};
use std::{
    ops::Mul,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub time: SystemTime,
    pub used_memory: u64,
    pub free_memory: u64,
    pub used_swap: u64,

    pub cpu_usage: f32,
    pub cpu_freq: u64,
}

impl SystemInfo {
    pub fn get_time_stamp(self: &Self) -> u128 {
        self.time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
    }

    pub fn memory_usage_percent(self: &Self) -> f32 {
        let free = self.free_memory as f32;
        let used = self.used_memory as f32;

        (free / used).mul(100f32)
    }
}
