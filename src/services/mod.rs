#[allow(unused)]
struct OperationService {}

#[allow(unused)]
impl OperationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find(&self, id: &str) {
        println!("find: {}", id);
    }
}
