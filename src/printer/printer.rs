use crate::config::config::Config;
use serde_derive::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Printer {
    pub config: Config,
}

#[derive(Deserialize)]
struct PrinterData {
    data: HashMap<String, Vec<HashMap<String, String>>>,
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
        fs::write(name, data)?;
        Ok(())
    }

    pub fn txt(&self, name: String, data: String) -> Result<(), Box<dyn Error>> {
        let v: PrinterData = serde_json::from_str(&data)?;
        let buf = String::new();

        println!("{:?}", v.data);

        fs::write(name, buf)?;

        Ok(())
    }
}
