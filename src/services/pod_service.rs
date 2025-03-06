pub struct Request {}

#[derive(Debug, Clone, Default)]
pub struct PodService;

impl PodService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn schedule(_request: Request) { // may return 429 if queue is full
                                         // - take request
                                         // - parse request
                                         // - enqueue task
                                         // - return operation id
    }
}
