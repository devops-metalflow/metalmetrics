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
        smol::block_on(async {
            let count = cpu::logical_count().await?;
            Ok(format!("{} CPU", count))
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
            let mut max: u64 = 0;
            let mut read: u64 = 0;
            let mut write: u64 = 0;
            let counters = disk::io_counters().await?;
            futures::pin_mut!(counters);

            while let Some(item) = counters.next().await {
                let item = item?;
                let r = item.read_bytes().get::<units::information::kilobyte>();
                let w = item.write_bytes().get::<units::information::kilobyte>();
                if r > max {
                    max = r;
                    read = r;
                    write = w;
                }
            }

            Ok(format!("RD {} KB WR {} KB", read, write))
        })
    }

    pub fn ip() -> Result<String, Box<dyn Error>> {
        smol::block_on(async {
            let mut buf: Vec<String> = vec![];
            let nic = net::nic().await?;
            futures::pin_mut!(nic);

            while let Some(item) = nic.next().await {
                let item = item?;
                if !item.is_loopback() && item.is_up() {
                    match item.address() {
                        net::Address::Inet(addr) => buf.push(addr.ip().to_string()),
                        _ => {}
                    };
                }
            }

            Ok(format!("{}", buf.join("\n")))
        })
    }

    pub fn kernel() -> Result<String, Box<dyn Error>> {
        // uname -sr
        smol::block_on(async {
            let platform = host::platform().await?;
            Ok(format!("{} {}", platform.system(), platform.release()))
        })
    }

    pub fn mac() -> Result<String, Box<dyn Error>> {
        smol::block_on(async {
            let helper = |address| match address {
                net::MacAddr::V6(buf) => buf.to_string(),
                _ => "".to_string(),
            };

            let mut buf: Vec<String> = vec![];
            let nic = net::nic().await?;
            futures::pin_mut!(nic);

            while let Some(item) = nic.next().await {
                let item = item?;
                if !item.is_loopback() && item.is_up() {
                    match item.address() {
                        net::Address::Link(addr) => buf.push(helper(addr)),
                        _ => {}
                    };
                }
            }

            Ok(format!("{}", buf.join("\n")))
        })
    }

    pub fn network() -> Result<String, Box<dyn Error>> {
        smol::block_on(async {
            let mut max: u64 = 0;
            let mut recv: u64 = 0;
            let mut sent: u64 = 0;
            let counters = net::io_counters().await?;
            futures::pin_mut!(counters);

            while let Some(item) = counters.next().await {
                let item = item?;
                let r = item.packets_recv();
                let s = item.packets_sent();
                if r > max {
                    max = r;
                    recv = r;
                    sent = s;
                }
            }

            Ok(format!("RX packets {} TX packets {}", recv, sent))
        })
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

        match String::from_utf8(output.stdout) {
            Ok(buf) => Ok(format!("{}", buf)),
            Err(_) => Err("invalid".into()),
        }
    }

    pub fn ram() -> Result<String, Box<dyn Error>> {
        // free -m | awk '/Mem/ {print $2}'
        // free -m | awk '/Mem/ {print $3}'
        smol::block_on(async {
            match memory::memory().await {
                Ok(buf) => {
                    let total = buf.total().get::<units::information::gigabyte>();
                    let available = buf.available().get::<units::information::gigabyte>();
                    Ok(format!("{} GB ({} GB Used)", total, (total - available)))
                }
                Err(_) => Err("invalid".into()),
            }
        })
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
