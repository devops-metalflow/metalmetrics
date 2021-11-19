use clap::{App, Arg};
use std::error::Error;

#[derive(Clone, Default)]
pub struct Argument {
    pub config_file: String,
    pub listen_url: String,
    pub output_file: String,
}

impl Argument {
    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        let matches = App::new("metalmetrics-rs")
            .version(concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build")))
            .arg(
                Arg::new("config_file")
                    .short('c')
                    .long("config-file")
                    .value_name("NAME")
                    .about("config file (.yml)")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::new("listen_url")
                    .short('l')
                    .long("listen-url")
                    .value_name("URL")
                    .about("listen url (host:port)")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("output_file")
                    .short('o')
                    .long("output-file")
                    .value_name("NAME")
                    .about("output file (.json|.txt)")
                    .takes_value(true)
                    .required(false)
                    .conflicts_with("listen_url"),
            )
            .get_matches();

        match matches.value_of("config_file") {
            Some(name) => self.config_file = name.to_string(),
            None => self.config_file = "".to_string(),
        }

        match matches.value_of("listen_url") {
            Some(url) => self.listen_url = url.to_string(),
            None => self.listen_url = "".to_string(),
        }

        match matches.value_of("output_file") {
            Some(name) => self.output_file = name.to_string(),
            None => self.output_file = "".to_string(),
        }

        Ok(())
    }
}
