use serde_derive::{Deserialize, Serialize};
use serde_yaml::{self};
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Default)]
pub struct Config {
    pub config_data: ConfigData,
    pub config_file: String,
    pub listen_url: String,
    pub output_file: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ConfigData {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: MetaData,
    pub spec: Spec,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct MetaData {
    pub name: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Spec {
    pub metrics: Vec<String>,
}

impl Config {
    pub fn build(&mut self) -> Result<(), Box<dyn Error>> {
        self.config()?;
        self.listen()?;
        self.output()?;

        Ok(())
    }

    pub fn config(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config_file.len() == 0 {
            return Err("name invalid".into());
        }

        if !self.config_file.ends_with(".yml") && !self.config_file.ends_with(".yaml") {
            return Err("suffix invalid".into());
        }

        if !Path::new(&self.config_file).exists() {
            return Err(format!("{} not found", self.config_file).into());
        }

        let f = File::open(&self.config_file)?;
        self.config_data = serde_yaml::from_reader(f)?;

        Ok(())
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        // PASS
        Ok(())
    }

    pub fn output(&mut self) -> Result<(), Box<dyn Error>> {
        if self.output_file.len() != 0 {
            if !self.output_file.ends_with(".json")
                && !self.output_file.ends_with(".txt")
                && !self.output_file.ends_with(".xlsx")
            {
                return Err("suffix invalid".into());
            }

            if Path::new(&self.output_file).exists() {
                return Err(format!("{} already exist", self.output_file).into());
            }
        }

        Ok(())
    }
}
