use crate::actor::Actor;

struct OperationScheduler {

}

enum Action {
    NewPod,
}

impl Actor for OperationScheduler {
    type Message = Action;

    #[async_trait]
    async fn handle(&mut self, ctx: &Context, message: Self::Message) -> Result<(), ActorError> {
	Ok(())
    }
}
