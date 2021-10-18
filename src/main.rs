use clap::{App, Arg};

fn main() {
    let app = App::new("metalmetrics-rs")
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
            Arg::new("inxi_file")
                .short('i')
                .long("inxi-file")
                .value_name("NAME")
                .about("inxi file (/path/to/inxi)")
                .takes_value(true)
                .required(false),
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
                .about("output file (.json|.txt|.xlsx)")
                .takes_value(true)
                .required(false)
                .conflicts_with("listen_url"),
        )
        .get_matches();
}
