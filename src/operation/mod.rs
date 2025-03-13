mod error;
mod operation;

#[derive(Debug)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn generate() -> Id {
        Id(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// struct OperationManagerActor {
//     operations: BTreeMap<Id, Operation>,
// }

// impl OperationManagerActor {
//     pub fn new() -> Self {
//         OperationmanagerActor {}
//     }
// }

// enum Message {
//     NewOperation,
//     WW,
// }

// #[async_trait]
// impl Actor for OperationManagerActor {
//     type Message = Message;
// }

// struct OperationManagerHandle {}

// impl OperationManagerHandle {
//     fn schedule() -> Id {}
//     fn new_transaction(id: Id) -> Result<Transaction, OperationError> {}
// }
