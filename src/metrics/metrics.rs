use crate::config::config::Config;
use futures::StreamExt;
use heim::{cpu, disk, host, net, units};
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
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $3}'
        smol::block_on(async {
            match cpu::physical_count().await {
                Ok(buf) => match buf {
                    Some(b) => Ok(format!("{} CPU", b)),
                    None => Err("failed to run cpu".into()),
                },
                Err(_) => Err("failed to run cpu".into()),
            }
        })
    }

    pub fn disk() -> Result<String, Box<dyn Error>> {
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $2}'
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $3}'
        smol::block_on(async {
            let usage = disk::usage(Path::new("/")).await?;
            let total = usage.total().get::<units::information::gigabyte>();
            let used = usage.used().get::<units::information::gigabyte>();
            Ok(format!(
                "{:.1} GB ({:.1} GB Used)",
                total as f64, used as f64
            ))
        })
    }

    pub fn io() -> Result<String, Box<dyn Error>> {
        smol::block_on(async {
            let counters = disk::io_counters().await?;
            futures::pin_mut!(counters);
            if let Ok(buf) = counters.next().await.ok_or("failed to run io")? {
                let read = buf.read_bytes().get::<units::information::kilobyte>();
                let write = buf.write_bytes().get::<units::information::kilobyte>();
                Ok(format!("RD {} KB WR {} KB", read, write))
            } else {
                Err("failed to run io".into())
            }
        })
    }

    pub fn ip() -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }

    pub fn kernel() -> Result<String, Box<dyn Error>> {
        // uname -sr
        smol::block_on(async {
            let platform = host::platform().await?;
            Ok(format!("{} {}", platform.system(), platform.release()))
        })
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
