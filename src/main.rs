mod arg;
mod config;

use arg::arg::Argument;
use config::config::Config;
use std::process;

fn main() {
    let mut args = Argument {
        ..Default::default()
    };

    if let Err(err) = args.parse() {
        println!("failed to parse argument: {}", err);
        process::exit(-1);
    }

    let mut cfg = Config {
        ..Default::default()
    };

    if let Err(err) = cfg.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }
}
