mod arg;
mod config;
mod flow;

use arg::arg::Argument;
use config::config::Config;
use flow::flow::Flow;
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
        config_file: args.config_file,
        inxi_file: args.inxi_file,
        listen_url: args.listen_url,
        output_file: args.output_file,
        ..Default::default()
    };

    if let Err(err) = cfg.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }
}
