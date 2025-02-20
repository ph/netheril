use tokio::sync::mpsc::{self, Receiver, Sender};

use super::{Actor, ActorError};

#[derive(Debug)]
#[repr(C)]
enum Priority {
    Normal = 1,
    High = 5,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Normal => write!(f, "Normal"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(Debug)]
pub struct Envelop<M: std::fmt::Debug> {
    priority: Priority,
    message: M,
}

impl<M: std::fmt::Debug> Envelop<M> {
    fn normal(message: M) -> Envelop<M> {
        Envelop {
            message,
            priority: Priority::Normal,
        }
    }

    fn high(message: M) -> Envelop<M> {
        Envelop {
            message,
            priority: Priority::High,
        }
    }
}

pub fn make_mailbox<A: Actor>() -> (Mailbox<A>, Inbox<A>) {
    let (normal_tx, normal_rx) = mpsc::channel::<Envelop<A::Message>>(1);
    let (high_tx, high_rx) = mpsc::channel::<Envelop<A::Message>>(1);

    (
        Mailbox::new(normal_tx, high_tx),
        Inbox::new(normal_rx, high_rx),
    )
}

#[derive(Debug, Clone)]
pub struct Mailbox<A: Actor> {
    normal_tx: Sender<Envelop<A::Message>>,
    high_tx: Sender<Envelop<A::Message>>,
}

impl<A: Actor> Mailbox<A> {
    fn new(normal_tx: Sender<Envelop<A::Message>>, high_tx: Sender<Envelop<A::Message>>) -> Self {
        Self { normal_tx, high_tx }
    }

    pub async fn send(&self, message: A::Message) -> Result<(), ActorError> {
        self.send_with_envelop(Envelop::normal(message)).await
    }

    pub async fn send_with_envelop(&self, message: Envelop<A::Message>) -> Result<(), ActorError> {
        self.routing(message).await
    }

    async fn routing(&self, message: Envelop<A::Message>) -> Result<(), ActorError> {
        match message.priority {
            Priority::Normal => self.normal_tx.send(message).await?,
            Priority::High => self.high_tx.send(message).await?,
        }

        Ok(())
    }
}

pub struct Inbox<A: Actor> {
    normal_rx: Receiver<Envelop<A::Message>>,
    high_rx: Receiver<Envelop<A::Message>>,
}

impl<A: Actor> Inbox<A> {
    fn new(
        normal_rx: Receiver<Envelop<A::Message>>,
        high_rx: Receiver<Envelop<A::Message>>,
    ) -> Self {
        Self { normal_rx, high_rx }
    }

    pub async fn recv(&mut self) -> Option<A::Message> {
        let message = tokio::select! {
            biased;
             Some(val) = self.high_rx.recv() => val.message,
             Some(val) = self.normal_rx.recv() => val.message,
        };

        Some(message)
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::actor::Context;

    use super::*;

    #[derive(Debug, Clone)]
    struct MyActor;

    #[async_trait]
    impl Actor for MyActor {
        type Message = Message;

        async fn handle(
            &mut self,
            _ctx: &Context,
            message: Self::Message,
        ) -> Result<(), ActorError> {
            match message {
                Message::Ping => println!("hello"),
                Message::Alert => println!("alert"),
            }
            Ok(())
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum Message {
        Ping,
        Alert,
    }

    #[tokio::test]
    async fn simple_send_and_receive() {
        let (normal_tx, normal_rx) = mpsc::channel(1);
        let (high_tx, high_rx) = mpsc::channel(1);

        let mailbox: Mailbox<MyActor> = Mailbox::new(normal_tx, high_tx);
        let mut inbox: Inbox<MyActor> = Inbox::new(normal_rx, high_rx);

        tokio::spawn(async move {
            let _ = mailbox.send(Message::Ping).await;
        });

        let message = inbox.recv().await.unwrap();

        assert_eq!(Message::Ping, message);
    }

    #[tokio::test]
    async fn send_and_receive_message_with_priority() {
        let (normal_tx, normal_rx) = mpsc::channel(1);
        let (high_tx, high_rx) = mpsc::channel(1);

        let mailbox: Mailbox<MyActor> = Mailbox::new(normal_tx, high_tx);
        let mailbox_2 = mailbox.clone();
        let mut inbox: Inbox<MyActor> = Inbox::new(normal_rx, high_rx);

        tokio::spawn(async move {
            let _ = mailbox.send(Message::Ping).await;
        });

        tokio::spawn(async move {
            let _ = mailbox_2
                .send_with_envelop(Envelop::high(Message::Alert))
                .await;
        });

        let mut messages = Vec::new();

        messages.push(inbox.recv().await.unwrap());
        messages.push(inbox.recv().await.unwrap());

        assert_eq!(2, messages.len());
        assert_eq!(vec![Message::Alert, Message::Ping,], messages);
    }
}
