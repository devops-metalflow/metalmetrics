#[test]
fn test_config() {
    let mut c = super::config::Config {
        config_file: "".to_string(),
        ..Default::default()
    };

    assert!(c.config().is_err());

    let mut c = super::config::Config {
        config_file: "config.txt".to_string(),
        ..Default::default()
    };

    assert!(c.config().is_err());

    let mut c = super::config::Config {
        config_file: "/config.yml".to_string(),
        ..Default::default()
    };

    assert!(c.config().is_err());

    let n = std::path::Path::new(&std::env::current_dir().unwrap())
        .join("src")
        .join("config")
        .join("config.yml");

    let mut c = super::config::Config {
        config_file: n.into_os_string().into_string().unwrap(),
        ..Default::default()
    };

    assert!(c.config().is_ok());
}

#[test]
fn test_listen() {
    let mut c = super::config::Config {
        listen_url: "".to_string(),
        ..Default::default()
    };

    assert!(c.listen().is_ok());
}

#[test]
fn test_output() {
    let mut c = super::config::Config {
        output_file: "".to_string(),
        ..Default::default()
    };

    assert!(c.output().is_ok());

    let mut c = super::config::Config {
        output_file: "output.foo".to_string(),
        ..Default::default()
    };

    assert!(c.output().is_err());

    let n = std::env::join_paths(&[
        std::path::Path::new(&std::env::current_dir().unwrap()),
        std::path::Path::new("config.yml"),
    ])
    .unwrap();

    let mut c = super::config::Config {
        output_file: n.into_string().unwrap(),
        ..Default::default()
    };

    assert!(c.output().is_err());

    let mut c = super::config::Config {
        output_file: "output.json".to_string(),
        ..Default::default()
    };

    assert!(c.output().is_ok());
}
