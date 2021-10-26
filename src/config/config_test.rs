#[test]
fn test_config() {
    let mut cfg = super::config::Config {
        config_file: "".to_string(),
        ..Default::default()
    };

    assert!(cfg.config().is_err());

    let mut cfg = super::config::Config {
        config_file: "config.txt".to_string(),
        ..Default::default()
    };

    assert!(cfg.config().is_err());

    let mut cfg = super::config::Config {
        config_file: "/config.yml".to_string(),
        ..Default::default()
    };

    assert!(cfg.config().is_err());

    let name = std::path::Path::new(&std::env::current_dir().unwrap())
        .join("src")
        .join("config")
        .join("config.yml");

    let mut cfg = super::config::Config {
        config_file: name.into_os_string().into_string().unwrap(),
        ..Default::default()
    };

    assert!(cfg.config().is_ok());
}

#[test]
fn test_inxi() {
    let mut cfg = super::config::Config {
        inxi_file: "".to_string(),
        ..Default::default()
    };

    assert!(cfg.inxi().is_ok());

    let mut cfg = super::config::Config {
        inxi_file: "foo".to_string(),
        ..Default::default()
    };

    assert!(cfg.inxi().is_err());
}

#[test]
fn test_listen() {
    let mut cfg = super::config::Config {
        listen_url: "".to_string(),
        ..Default::default()
    };

    assert!(cfg.listen().is_ok());
}

#[test]
fn test_output() {
    let mut cfg = super::config::Config {
        output_file: "".to_string(),
        ..Default::default()
    };

    assert!(cfg.output().is_ok());

    let mut cfg = super::config::Config {
        output_file: "output.foo".to_string(),
        ..Default::default()
    };

    assert!(cfg.output().is_err());

    let name = std::env::join_paths(&[
        std::path::Path::new(&std::env::current_dir().unwrap()),
        std::path::Path::new("config.yml"),
    ])
    .unwrap();

    let mut cfg = super::config::Config {
        output_file: name.into_string().unwrap(),
        ..Default::default()
    };

    assert!(cfg.output().is_err());

    let mut cfg = super::config::Config {
        output_file: "output.json".to_string(),
        ..Default::default()
    };

    assert!(cfg.output().is_ok());
}
