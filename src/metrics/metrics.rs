use crate::config::config::Config;
use std::error::Error;

#[derive(Default)]
pub struct Metrics {}

impl Metrics {
    pub fn routine(_cfg: Config, _spec: Option<&str>) -> Result<String, Box<dyn Error>> {
        // TODO
        Ok("".to_string())
    }
}
