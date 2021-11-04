use std::error::Error;

#[derive(Debug, Default)]
pub struct Metrics {}

impl Metrics {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }
}
