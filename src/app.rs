use tokio::sync::broadcast::{self, Receiver, Sender};
use tracing::info;

use crate::{
    api::router,
    error::NetherilErr,
    logging::{Logging, LoggingOptions}, services::{operation_service::OperationService, pod_service::PodService, ServiceRegistry},
};

pub struct App {
    #[allow(dead_code)]
    logging: Logging,
}

#[derive(Debug, Clone)]
enum Broadcast {
    Interrupt,
}

impl std::fmt::Display for Broadcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Broadcast::*;

        match self {
            Interrupt => write!(f, "broadcast interrupt"),
        }
    }
}

impl App {
    pub fn new() -> Self {
        info!("configuring");
        let logging = Logging::new(LoggingOptions::default());
        App { logging }
    }

    fn configure_services(&self) -> ServiceRegistry{
	info!("configure services");

        ServiceRegistry {
            operation_service: OperationService::new(),
	    pod_service: PodService::new(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<NetherilErr>> {
        info!("starting");

        let services = self.configure_services();

        let router = router().with_state(services);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .map_err(|e| NetherilErr::Api(e.to_string()))?;

        let (broadcast, rx) = broadcast::channel::<Broadcast>(1);

        let mut handles = Vec::new();

        let handle = tokio::spawn(async move {
            let _ = register_signals(broadcast).await;
        });
        handles.push(handle);

        let handle = tokio::spawn(async move {
            let rx = rx.resubscribe();

            axum::serve(listener, router)
                .with_graceful_shutdown(handle_shutdown_signal(rx))
                .await
                .map_err(|e| NetherilErr::Api(e.to_string()))
                .unwrap();
        });
        handles.push(handle);

        for handle in handles {
            handle.await.unwrap();
        }

        Ok(())
    }
}

async fn handle_shutdown_signal(mut receiver: Receiver<Broadcast>) {
    match receiver.recv().await {
        Ok(Broadcast::Interrupt) | Err(_) => (),
    }
}

async fn register_signals(broadcast: Sender<Broadcast>) -> Result<(), Box<dyn std::error::Error>> {
    let mut interrupt_count = 0;
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
            interrupt_count += 1;
            if interrupt_count > 1 {
                broadcast.send(Broadcast::Interrupt)?;
                return Ok(())
            }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
