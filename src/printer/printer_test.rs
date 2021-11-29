#[allow(unused_imports)]
use std::fs;

#[allow(unused_imports)]
use crate::config::config::{Config, METRICS, SEP};

#[test]
fn test_run() {
    let name = "invalid";

    let c = Config {
        output_file: name.to_string(),
        ..Default::default()
    };

    let p = super::printer::Printer { config: c.clone() };

    match p.run("test".to_string()) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    };
}

#[test]
fn test_json() {
    let name = "output.json";

    let c = Config {
        output_file: name.to_string(),
        ..Default::default()
    };

    let p = super::printer::Printer { config: c.clone() };
    let buf = String::from("{\"metrics\":[{\"cpu\":\"16 CPU\"}]}");

    match p.run(buf) {
        Ok(_) => {
            match fs::remove_file(name) {
                Ok(_) => {}
                Err(_) => {}
            };
            assert!(true)
        }
        Err(_) => assert!(false),
    };
}

#[test]
fn test_txt() {
    let name = "output.txt";

    let c = Config {
        output_file: name.to_string(),
        ..Default::default()
    };

    let p = super::printer::Printer { config: c.clone() };
    let buf = String::from("{\"metrics\":[{\"cpu\":\"16 CPU\"}]}");

    match p.run(buf) {
        Ok(_) => {
            match fs::remove_file(name) {
                Ok(_) => {}
                Err(_) => {}
            };
            assert!(true)
        }
        Err(_) => assert!(false),
    };
}
