use crate::actor::Actor;

struct OperationScheduler {}

impl Actor for OperationScheduler {
    type Message;

    #[async_trait]
    async fn handle(&mut self, ctx: &Context, message: Self::Message) -> Result<(), ActorError> {
	Ok(())
    }
}
