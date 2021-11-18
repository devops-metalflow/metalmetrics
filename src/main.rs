mod arg;
mod config;
mod flow;
mod metrics;

use arg::arg::Argument;
use config::config::{Config, PREFIX};
use flow::flow::Flow;
use metrics::metrics::Metrics;
use std::process;

#[tokio::main]
async fn main() {
    let mut a = Argument {
        ..Default::default()
    };

    if let Err(err) = a.parse() {
        println!("failed to parse argument: {}", err);
        process::exit(-1);
    }

    let mut c = Config {
        config_file: a.config_file,
        listen_url: a.listen_url,
        output_file: a.output_file,
        ..Default::default()
    };

    if let Err(err) = c.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }

    println!("metrics running");

    if c.listen_url.len() != 0 {
        let f = Flow {
            config: c,
            routine: Metrics::routine,
        };

        if let Err(err) = f.run().await {
            println!("failed to run flow: {}", err);
            process::exit(-3);
        }
    } else {
        match Metrics::routine(c, PREFIX) {
            Ok(buf) => println!("{}", buf),
            Err(err) => {
                println!("failed to run metrics: {}", err);
                process::exit(-4);
            }
        }
    }

    println!("metrics exiting");
}
