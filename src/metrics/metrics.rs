use crate::config::config::{Config, METRICS, NAME, SEP};
use futures::StreamExt;
use futures_timer::Delay;
use heim::{cpu, disk, host, memory, net, units};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::net::UdpSocket;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub struct Metrics {}

impl Metrics {
    pub fn routine(cfg: Config, spec: &str) -> Result<String, Box<dyn Error>> {
        let helper = |spec| match spec {
            "assets" => Metrics::assets(),
            "cpu" => Metrics::cpu(),
            "disk" => Metrics::disk(),
            "io" => Metrics::io(),
            "ip" => Metrics::ip(),
            "kernel" => Metrics::kernel(),
            "mac" => Metrics::mac(),
            "network" => Metrics::network(),
            "os" => Metrics::os(),
            "ram" => Metrics::ram(),
            "users" => Metrics::users(),
            &_ => Err("spec invalid".into()),
        };

        if spec.len() == 0 || !spec.starts_with(METRICS) {
            return Err("spec invalid".into());
        }

        let mut s: Vec<String> = vec![];

        if spec == METRICS {
            s = cfg.config_data.spec.metrics;
        } else {
            let mut b = String::from(METRICS);
            b.push_str(SEP);
            let b = spec.trim_start_matches(&b);
            s.push(b.to_string());
        }

        let mut tuples: Vec<(String, String)> = Vec::new();

        for item in &s {
            let m = helper(item)?;
            tuples.push((item.to_string(), m))
        }

        let t: HashMap<_, _> = tuples.into_iter().collect();

        let mut buf: HashMap<_, _> = HashMap::new();
        let mut b: Vec<HashMap<_, _>> = Vec::new();

        b.push(t);
        buf.insert(NAME.to_string(), b);

        Ok(format!("{:?}", buf))
    }

    pub fn assets() -> Result<String, Box<dyn Error>> {
        // cat /assets/assets/assets.ini
        let helper = |data: String| -> String {
            match data.strip_prefix("assetsNo=") {
                Some(b) => b.parse().unwrap(),
                None => "".to_string(),
            }
        };

        let contents = fs::read_to_string("/assets/assets/assets.ini");
        if contents.is_err() {
            return Ok("".to_string());
        }

        let contents = contents.unwrap();
        let lines = contents.lines();
        let mut buf = String::new();

        for item in lines {
            buf = helper(item.parse().unwrap());
            if !buf.is_empty() {
                break;
            }
        }

        Ok(format!("{}", buf))
    }

    pub fn cpu() -> Result<String, Box<dyn Error>> {
        // awk -F: '/model name/ {core++} END {print core}' /proc/cpuinfo
        smol::block_on(async {
            let count = cpu::logical_count().await?;

            let measure_before = cpu::usage().await?;
            Delay::new(Duration::from_millis(100)).await;
            let measure_after = cpu::usage().await?;
            let percent = (measure_after - measure_before).get::<units::ratio::percent>();

            Ok(format!("{} CPU ({}% Used)", count, percent))
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
            let socket = UdpSocket::bind("0.0.0.0:0")?;
            socket.connect("8.8.8.8:8")?;
            let addr = socket.local_addr().unwrap();
            Ok(format!("{}", addr.ip().to_string()))
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
            let helper = |data| match data {
                net::MacAddr::V6(buf) => buf.to_string(),
                _ => "".to_string(),
            };

            let mut name: String = "".to_string();
            let ip = Metrics::ip()?;
            let nic = net::nic().await?;
            futures::pin_mut!(nic);

            while let Some(item) = nic.next().await {
                let item = item?;
                match item.address() {
                    net::Address::Inet(addr) => {
                        if addr.ip().to_string() == ip {
                            name = item.name().parse().unwrap();
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let mut buf: String = "".to_string();
            let nic = net::nic().await?;
            futures::pin_mut!(nic);

            while let Some(item) = nic.next().await {
                let item = item?;
                if item.name().to_string() == name {
                    match item.address() {
                        net::Address::Link(addr) => {
                            buf = helper(addr);
                            break;
                        }
                        _ => {}
                    };
                }
            }

            Ok(format!("{}", buf))
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
            Ok(buf) => Ok(format!("{}", buf.trim())),
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
        let helper = |mut data: std::str::Lines| {
            let mut buf: Vec<String> = vec![];
            while let Some(item) = data.next() {
                let collect: Vec<&str> = item.split(":").collect();
                let val = collect[2].parse::<i32>().unwrap();
                if val >= 1000 && val <= 60000 {
                    buf.push(collect[0].to_string());
                }
            }
            format!("{}", buf.join("\n"))
        };

        let output = Command::new("getent").arg("passwd").output()?;
        if !output.status.success() {
            return Err("invalid".into());
        }

        match String::from_utf8(output.stdout) {
            Ok(buf) => Ok(helper(buf.lines())),
            Err(_) => Err("invalid".into()),
        }
    }
}
