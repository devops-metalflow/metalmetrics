use crate::config::config::Config;
use psutil::{cpu, disk};
use std::error::Error;
use std::path::Path;

pub struct Metrics {}

impl Metrics {
    pub fn routine(_cfg: Config, _spec: Option<&str>) -> Result<String, Box<dyn Error>> {
        // TODO
        Ok("".to_string())
    }

    pub fn cpu() -> Result<String, Box<dyn Error>> {
        // awk -F: '/model name/ {core++} END {print core}' /proc/cpuinfo
        Ok(format!("{} CPU", cpu::cpu_count()))
    }

    pub fn disk() -> Result<String, Box<dyn Error>> {
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $2}'
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $3}'
        let usage = disk::disk_usage(Path::new("/"))?;
        Ok(format!(
            "{:.1} GB ({:.1} GB Used)",
            (usage.total() >> 30) as f64,
            (usage.used() >> 30) as f64
        ))
    }

    pub fn io() -> Result<String, Box<dyn Error>> {
        let mut collector = disk::DiskIoCountersCollector::default();
        let read = collector.disk_io_counters().unwrap().read_bytes() >> 10;
        let write = collector.disk_io_counters().unwrap().write_bytes() >> 10;
        Ok(format!("RD {} KB WR {} KB", read, write))
    }

    pub fn ip() -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }

    pub fn kernel() -> Result<String, Box<dyn Error>> {
        // uname -r
        Ok("".to_string())
    }

    pub fn mac() -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }

    pub fn network() -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }

    pub fn os() -> Result<String, Box<dyn Error>> {
        // awk -F'[= "]' '/PRETTY_NAME/{print $3,$4,$5}' /etc/os-release
        Ok("".to_string())
    }

    pub fn ram() -> Result<String, Box<dyn Error>> {
        // free -m | awk '/Mem/ {print $2}'
        // free -m | awk '/Mem/ {print $3}'
        Ok("".to_string())
    }

    pub fn system() -> Result<String, Box<dyn Error>> {
        // perl ./inxi -F
        Ok("".to_string())
    }

    pub fn users() -> Result<String, Box<dyn Error>> {
        // getent passwd {1000..60000}
        Ok("".to_string())
    }
}
