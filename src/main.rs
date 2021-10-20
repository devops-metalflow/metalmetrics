mod cmd;

fn main() {
    let mut arg = cmd::argument::Argument {
        config_file: "".to_string(),
        inxi_file: "".to_string(),
        listen_url: "".to_string(),
        output_file: "".to_string(),
    };

    arg.parse();
}
