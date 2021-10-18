use clap::{App, Arg};

fn main() {
    let app = App::new("metalmetrics-rs")
        .version(concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build")))
        .arg(
            Arg::new("config_file")
                .long("config-file")
                .short('c')
                .value_name("NAME")
                .about("config file (.yml)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("inxi_file")
                .long("inxi-file")
                .short('i')
                .value_name("NAME")
                .about("inxi file (/path/to/inxi)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("listen_url")
                .long("listen-url")
                .short('l')
                .value_name("URL")
                .about("listen url (host:port)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .short('o')
                .value_name("NAME")
                .about("output file (.json|.txt|.xlsx)")
                .takes_value(true)
                .required(false)
                .conflicts_with("listen_url"),
        )
        .get_matches();
}
