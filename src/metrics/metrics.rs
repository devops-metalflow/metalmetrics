use crate::config::config::{Config, METRICS, NAME, SEP};
use futures::StreamExt;
use futures_timer::Delay;
use heim::{cpu, disk, host, memory, net, units};
use procfs::process::Process;
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
            "eth" => Metrics::eth(),
            "io" => Metrics::io(),
            "ip" => Metrics::ip(),
            "kernel" => Metrics::kernel(),
            "mac" => Metrics::mac(),
            "network" => Metrics::network(),
            "os" => Metrics::os(),
            "ram" => Metrics::ram(),
            "users" => Metrics::users(),
            "wake" => Metrics::wake(),
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

        let mut buf = String::new();
        let contents = contents.unwrap();
        let lines = contents.lines();

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
            Delay::new(Duration::from_millis(1000)).await;
            let measure_after = cpu::usage().await?;
            let percent = (measure_after - measure_before).get::<units::ratio::percent>();

            Ok(format!("{} CPU ({}% Used)", count, percent as u64 / count))
        })
    }

    pub fn disk() -> Result<String, Box<dyn Error>> {
        // cat /proc/self/mountinfo
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $2}'
        // df -hPl | grep -wvE '\\-|none|tmpfs|devtmpfs|by-uuid|chroot|Filesystem|udev|docker' | awk '{print $3}'
        smol::block_on(async {
            let mut total: u64 = 0;
            let mut used: u64 = 0;

            for mount in Process::myself().unwrap().mountinfo().unwrap() {
                if mount.fs_type == "ext4" {
                    let usage =
                        disk::usage(Path::new(&mount.mount_point.display().to_string())).await?;
                    let t = usage.total().get::<units::information::gigabyte>();
                    let u = usage.used().get::<units::information::gigabyte>();
                    total += t;
                    used += u;
                }
            }

            Ok(format!(
                "{:.1} GB ({:.1} GB Used)",
                total as f64, used as f64
            ))
        })
    }

    pub fn eth() -> Result<String, Box<dyn Error>> {
        smol::block_on(async {
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

            Ok(format!("{}", name))
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

            let mut buf: String = "".to_string();
            let name = Metrics::eth()?;
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

    pub fn wake() -> Result<String, Box<dyn Error>> {
        // sudo apt install ethtool
        // sudo ethtool eth0
        //
        // Output Example
        // Supports Wake-on: pumbg
        // Wake-on: d
        //
        // Option  Description
        // b       Wake on broadcast messages
        // d       Wake on nothing
        // g       Wake on MagicPacket messages
        // m       Wake on multicast messages
        // p       Wake on PHY activity
        // u       Wake on unicast messages
        let helper = |mut data: std::str::Lines| {
            let mut buf: Vec<String> = vec![];
            while let Some(item) = data.next() {
                let collect: Vec<&str> = item.split(":").collect();
                let key = collect[0].parse::<String>().unwrap();
                if key.contains("Wake-on") {
                    buf.push(item.to_string().trim().parse().unwrap());
                }
            }
            format!("{}", buf.join("\n"))
        };

        let name = Metrics::eth()?;

        let output = Command::new("ethtool").arg(name).output()?;
        if !output.status.success() {
            return Err("invalid".into());
        }

        match String::from_utf8(output.stdout) {
            Ok(buf) => Ok(helper(buf.lines())),
            Err(_) => Err("invalid".into()),
        }
    }
}
