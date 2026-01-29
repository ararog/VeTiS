use crate::config::{ListenerConfig, Protocol, SecurityConfig, ServerConfig, VirtualHostConfig};

#[test]
fn test_listener_config() {
    let listener_config = ListenerConfig::builder()
        .port(8080)
        .ssl(false)
        .protocol(Protocol::HTTP1)
        .interface("127.0.0.1".to_string())
        .build();
    assert_eq!(listener_config.port(), 8080);
    assert!(!listener_config.ssl());
    assert_eq!(listener_config.protocol(), &Protocol::HTTP1);
    assert_eq!(listener_config.interface(), "127.0.0.1");
}

#[test]
fn test_server_config() {
    let server_config = ServerConfig::builder()
        .add_listener(
            ListenerConfig::builder()
                .port(8080)
                .build(),
        )
        .build();
    assert_eq!(
        server_config
            .listeners()
            .len(),
        1
    );
}

#[test]
fn test_security_config() {
    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(vec![])
        .cert_from_bytes(vec![])
        .key_from_bytes(vec![])
        .build();
    assert!(security_config
        .ca_cert()
        .is_some());
    assert!(security_config
        .cert()
        .is_empty());
    assert!(security_config
        .key()
        .is_empty());
}

#[test]
fn test_virtual_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let virtual_host_config = VirtualHostConfig::builder()
        .hostname("localhost".to_string())
        .port(8080)
        .build()?;
    assert_eq!(virtual_host_config.hostname(), "localhost");
    assert_eq!(virtual_host_config.port(), 8080);

    Ok(())
}
