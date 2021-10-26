#[test]
fn test_argument() {
    let args = super::arg::Argument {
        ..Default::default()
    };

    assert_eq!(args.config_file.is_empty(), true);
}
