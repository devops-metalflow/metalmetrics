#[test]
fn test_argument() {
    let args = super::arg::Argument {
        config_file: "".to_string(),
        inxi_file: "".to_string(),
        listen_url: "".to_string(),
        output_file: "".to_string(),
    };

    assert_eq!(args.config_file.is_empty(), true);
}
