mod arg;
use arg::arg::Argument;

fn main() {
    let mut args = Argument {
        config_file: "".to_string(),
        inxi_file: "".to_string(),
        listen_url: "".to_string(),
        output_file: "".to_string(),
    };

    args.parse();
}
