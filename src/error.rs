#[derive(Debug, Clone)]
pub enum NetherilErr {
    #[allow(dead_code)]
    Logging(String),
    Api(String),
    PodConfigurationError(String),
    UnknownPodProvider(String),
    Runner(String),
}

impl std::error::Error for NetherilErr {}

impl std::fmt::Display for NetherilErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use NetherilErr::*;

        match self {
            Logging(e) => write!(f, "logging error: {}", e),
            Api(e) => write!(f, "api error: {}", e),
            PodConfigurationError(e) => write!(f, "pod configuration error: {}", e),
            UnknownPodProvider(provider) => write!(f, "unknown pod provider named: {}", provider),
            Runner(e) => write!(f, "runner error: {}", e),
        }
    }
}
