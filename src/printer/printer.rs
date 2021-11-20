use crate::config::config::Config;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Printer {
    pub config: Config,
}

impl Printer {
    pub fn run(&self, data: String) -> Result<(), Box<dyn Error>> {
        let name = &self.config.output_file;

        if name.ends_with(".json") {
            self.json(name.to_string(), data)?;
        } else if name.ends_with(".txt") {
            self.txt(name.to_string(), data)?;
        } else {
            return Err("suffix invalid".into());
        }

        Ok(())
    }

    pub fn json(&self, name: String, data: String) -> Result<(), Box<dyn Error>> {
        let mut buf = String::new();

        buf.push_str(&data);
        buf.push('\n');

        fs::write(name, buf)?;

        Ok(())
    }

    pub fn txt(&self, name: String, data: String) -> Result<(), Box<dyn Error>> {
        let helper = |data| {
            let mut buf = String::new();
            for (key, val) in data {
                buf.push_str(&format!("{}: {}", key, val));
                buf.push('\n');
            }
            buf
        };

        let v: HashMap<String, Vec<HashMap<String, String>>> = serde_json::from_str(&data)?;
        let mut buf = String::new();

        for (key, val) in v {
            buf.push_str(&key);
            buf.push('\n');
            buf.push_str(&helper(val[0].clone()));
        }

        fs::write(name, buf)?;

        Ok(())
    }
}
