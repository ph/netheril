#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Queued,
    Working,
    Failed,
    Canceled,
    Completed,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Queued => write!(f, "queued"),
            State::Working => write!(f, "working"),
            State::Failed => write!(f, "failed"),
            State::Canceled => write!(f, "canceled"),
            State::Completed => write!(f, "completed"),
        }
    }
}
