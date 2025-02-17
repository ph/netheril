#[derive(Debug, Clone)]
pub enum NetherilErr {
    Logging(String),
}

impl std::error::Error for NetherilErr {}

impl std::fmt::Display for NetherilErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetherilErr::Logging(e) => write!(f, "logging error: {}", e),
        }
    }
}
