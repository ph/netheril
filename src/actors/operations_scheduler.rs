use async_trait::async_trait;

use crate::actor::{Actor, ActorError, Context};

struct OperationScheduler;

#[derive(Debug)]
enum Action {
    NewPod,
}

#[async_trait]
impl Actor for OperationScheduler {
    type Message = Action;

    async fn handle(&mut self, _ctx: &Context, _message: Self::Message) -> Result<(), ActorError> {
	Ok(())
    }
}
