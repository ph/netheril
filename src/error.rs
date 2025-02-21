#[derive(Debug, Clone)]
pub enum NetherilErr {
    #[allow(dead_code)]
    Logging(String),
    Api(String),
}

impl std::error::Error for NetherilErr {}

impl std::fmt::Display for NetherilErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use NetherilErr::*;

        match self {
            Logging(e) => write!(f, "logging error: {}", e),
            Api(e) => write!(f, "api error: {}", e),
        }
    }
}
