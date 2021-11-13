use crate::config::config::Config;
use futures::StreamExt;
use heim::{cpu, disk, host, memory, net, units};
use std::error::Error;
use std::path::Path;
use std::process::Command;

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
                    None => Err("invalid".into()),
                },
                Err(_) => Err("invalid".into()),
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

            if let Ok(buf) = counters.next().await.ok_or("invalid")? {
                let read = buf.read_bytes().get::<units::information::kilobyte>();
                let write = buf.write_bytes().get::<units::information::kilobyte>();
                Ok(format!("RD {} KB WR {} KB", read, write))
            } else {
                Err("invalid".into())
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
        let output = Command::new("awk")
            .arg("-F[= \"]")
            .arg("/PRETTY_NAME/{print $3,$4,$5}")
            .arg("/etc/os-release")
            .output()?;
        if !output.status.success() {
            return Err("invalid".into());
        }

        Ok(format!("{:?}", String::from_utf8(output.stdout)))
    }

    pub fn ram() -> Result<String, Box<dyn Error>> {
        // free -m | awk '/Mem/ {print $2}'
        // free -m | awk '/Mem/ {print $3}'
        smol::block_on(async {
            match memory::memory().await {
                Ok(buf) => {
                    let total = buf.total().get::<units::information::megabyte>();
                    let free = buf.free().get::<units::information::megabyte>();
                    Ok(format!("{} MB ({} MB Used)", total, (total - free)))
                }
                Err(_) => Err("invalid".into()),
            }
        })
    }

    pub fn system(name: String) -> Result<String, Box<dyn Error>> {
        // perl ./inxi -F
        let output = Command::new("perl").arg(name).arg("-F").output()?;
        if !output.status.success() {
            return Err("invalid".into());
        }

        Ok(format!("{:?}", String::from_utf8(output.stdout)))
    }

    pub fn users() -> Result<String, Box<dyn Error>> {
        // getent passwd {1000..60000}
        smol::block_on(async {
            let mut buf: Vec<String> = vec![];
            let users = host::users().await?;
            futures::pin_mut!(users);

            while let Some(item) = users.next().await {
                let item = item?;
                buf.push(item.username().to_string());
            }

            Ok(format!("{}", buf.join("\n")))
        })
    }
}
