pub struct Transaction {
    id: Id,
}

impl Transaction {
    pub fn new(id: Id) -> Transaction {
        Transaction { id: Id }
    }

    pub fn start() {}
    pub fn fail(_error: OperationError) {}
    pub fn cancel(_reason: String) {}
    pub fn complete() {}
}
