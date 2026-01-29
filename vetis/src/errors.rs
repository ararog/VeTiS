use thiserror::Error;

#[derive(Debug, Error)]
pub enum VetisError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Failed to bind to address: {0}")]
    Bind(String),

    #[error("Failed to start server: {0}")]
    Start(#[from] StartError),

    #[error("Failed to stop server: {0}")]
    Stop(String),

    #[error("Handler error: {0}")]
    Handler(String),

    #[error("Tls error: {0}")]
    Tls(String),

    #[error("No instances")]
    NoInstances,

    #[error("Virtual host error: {0}")]
    VirtualHost(#[from] VirtualHostError),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ConfigError {
    #[error("Invalid virtual host config: {0}")]
    VirtualHost(String),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum StartError {
    #[error("Tls initialization: {0}")]
    Tls(String),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum VirtualHostError {
    #[error("No virtual hosts")]
    NoVirtualHosts,
}
