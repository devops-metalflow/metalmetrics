mod arg;
mod config;
mod flow;
mod metrics;

use arg::arg::Argument;
use config::config::Config;
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
        inxi_file: a.inxi_file,
        listen_url: a.listen_url,
        output_file: a.output_file,
        ..Default::default()
    };

    if let Err(err) = c.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }

    let mut m = Metrics {
        ..Default::default()
    };

    if let Err(err) = m.run() {
        println!("failed to run metrics: {}", err);
        process::exit(-3);
    }

    println!("metrics running");

    if c.listen_url.len() != 0 {
        let f = Flow {
            config: c,
            ..Default::default()
        };

        if let Err(err) = f.run().await {
            println!("failed to run flow: {}", err);
            process::exit(-4);
        }
    } else {
        // TODO
    }

    println!("metrics exiting");
}
