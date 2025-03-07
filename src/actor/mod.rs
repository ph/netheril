#![allow(unused)]
use async_trait::async_trait;
use mailbox::{make_mailbox, Inbox, Mailbox};
use tokio::{sync::mpsc::error::SendError, task::JoinHandle};
use uuid::Uuid;

pub mod mailbox;

#[derive(Debug, Clone)]
pub enum ActorError {
    Send(String),
    Quit,
}

impl std::error::Error for ActorError {}
impl std::fmt::Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorError::Send(s) => write!(f, "Send error: {}", s),
            ActorError::Quit => write!(f, "quit"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    id: Uuid,
}

impl Context {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[async_trait]
pub trait Actor: Send {
    type Message: Send + Sync + 'static;

    async fn handle(&mut self, ctx: &Context, message: Self::Message) -> Result<(), ActorError>;
}

#[derive(Debug)]
pub struct Workspace {
    name: String,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace::new("default")
    }
}

impl Workspace {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn spawn<A: Actor + 'static>(&self, actor: A) -> (Mailbox<A>, JoinHandle<()>) {
        let (mailbox, inbox) = make_mailbox();

        let ctx = Context::new();
        let handle = tokio::spawn(execution_loop(actor, inbox, ctx.clone()));

        (mailbox, handle)
    }
}

async fn execution_loop<A: Actor>(mut actor: A, mut inbox: Inbox<A>, ctx: Context) {
    while let Some(message) = inbox.recv().await {
        match actor.handle(&ctx, message).await {
            Ok(_) => {}
            Err(ActorError::Quit) => return,
            Err(_) => return,
        }
    }
}

impl<T> From<SendError<T>> for ActorError {
    fn from(value: SendError<T>) -> Self {
        ActorError::Send(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::oneshot;

    use super::*;

    #[derive(Debug)]
    enum Message {
        Increment,
        Decrement,
        GetCounter(oneshot::Sender<isize>),
        Quit,
    }

    #[derive(Debug, Default, Clone)]
    struct CounterActor {
        count: isize,
    }

    #[async_trait]
    impl Actor for CounterActor {
        type Message = Message;

        async fn handle(
            &mut self,
            _ctx: &Context,
            message: Self::Message,
        ) -> Result<(), ActorError> {
            match message {
                Message::Increment => self.count += 1,
                Message::Decrement => self.count -= 1,
                Message::Quit => return Err(ActorError::Quit),
                Message::GetCounter(reply_to) => {
                    let _ = reply_to.send(self.count);
                }
	    }

            Ok(())
        }
    }

    #[tokio::test]
    async fn send_and_receive_multiple_messages() {
        let workspace = Workspace::default();
        let counter = CounterActor::default();
        let (mailbox, actor_handle) = workspace.spawn(counter);

        let (tx, rx) = oneshot::channel::<isize>();

        tokio::spawn(async move {
            let ops = vec![
                Message::Increment,
                Message::Decrement,
                Message::Increment,
                Message::Increment,
                Message::GetCounter(tx),
                Message::Quit,
            ];

            for op in ops {
                mailbox.send(op).await.unwrap();
            }
        });
        let _ = actor_handle.await;

        let v = rx.await.unwrap();
        assert_eq!(2, v);
    }
}
