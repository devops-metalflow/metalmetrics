#[test]
fn test_routine() {
    // TODO
}

#[test]
fn test_cpu() {
    match super::metrics::Metrics::cpu() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_disk() {
    match super::metrics::Metrics::disk() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_io() {
    match super::metrics::Metrics::io() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_ip() {
    match super::metrics::Metrics::ip() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_kernel() {
    match super::metrics::Metrics::kernel() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_mac() {
    match super::metrics::Metrics::mac() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_network() {
    match super::metrics::Metrics::network() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_os() {
    match super::metrics::Metrics::os() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_ram() {
    match super::metrics::Metrics::ram() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_users() {
    match super::metrics::Metrics::users() {
        Ok(buf) => {
            println!("{}", buf);
            assert!(buf.len() != 0);
        }
        Err(_) => assert!(false),
    }
}
