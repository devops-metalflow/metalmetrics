#[allow(unused_imports)]
use crate::config::config::{Config, METRICS, SEP};

#[test]
fn test_routine() {
    let helper = || {
        let n = std::path::Path::new(&std::env::current_dir().unwrap())
            .join("src")
            .join("config")
            .join("config.yml");
        let mut c = Config {
            config_file: n.into_os_string().into_string().unwrap(),
            listen_url: "".to_string(),
            output_file: "".to_string(),
            ..Default::default()
        };
        match c.build() {
            Ok(_) => {}
            Err(_) => {}
        };
        c
    };

    if let Err(_) = super::metrics::Metrics::routine(helper(), "") {
        assert!(true);
    }

    if let Err(_) = super::metrics::Metrics::routine(helper(), "invalid") {
        assert!(true);
    }

    match super::metrics::Metrics::routine(helper(), METRICS) {
        Ok(buf) => {
            println!("{}", buf);
            assert_ne!(buf.len(), 0);
        }
        Err(_) => assert!(false),
    }

    let mut spec = METRICS.to_string();
    spec += SEP;
    spec += "cpu";

    match super::metrics::Metrics::routine(helper(), &spec) {
        Ok(buf) => {
            println!("{}", buf);
            assert_ne!(buf.len(), 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_assets() {
    match super::metrics::Metrics::assets() {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_cpu() {
    match super::metrics::Metrics::cpu() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_disk() {
    match super::metrics::Metrics::disk() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_io() {
    match super::metrics::Metrics::io() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_ip() {
    match super::metrics::Metrics::ip() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_kernel() {
    match super::metrics::Metrics::kernel() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_mac() {
    match super::metrics::Metrics::mac() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_network() {
    match super::metrics::Metrics::network() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_os() {
    match super::metrics::Metrics::os() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_ram() {
    match super::metrics::Metrics::ram() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_users() {
    match super::metrics::Metrics::users() {
        Ok(buf) => assert_ne!(buf.len(), 0),
        Err(_) => assert!(false),
    }
}
