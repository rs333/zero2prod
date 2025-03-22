use zero2prod::configuration::ApplicationSettings;

#[test]
fn application_settings_port_from_str() {
    let s = r#" { "port": "123", "host": "Host" } "#;
    let a: ApplicationSettings = serde_json::from_str(s).unwrap();
    assert_eq!(a.port, 123);
}

#[test]
fn application_settings_port_from_int() {
    let s = r#" { "port": 444 , "host": "Host" } "#;
    let a: ApplicationSettings = serde_json::from_str(s).unwrap();
    assert_eq!(a.port, 444);
}
