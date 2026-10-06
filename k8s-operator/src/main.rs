use futures::StreamExt;
use kube::{
    Client, CustomResource, ResourceExt,
    api::Api,
    runtime::{
        controller::{Action, Controller},
        watcher,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::Duration;
use tracing::{error, info, instrument};

// Custom Resource Definition with explicit plural and singular routing
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "metalbear.example.com",
    version = "v1",
    kind = "Echo",
    plural = "echoes",
    singular = "echo",
    namespaced
)]
pub struct EchoSpec {
    pub message: String,
}

pub struct ContextData {
    client: Client,
}

// Reconciliation loop function
#[instrument(skip(ctx, echo))]
async fn reconcile(echo: Arc<Echo>, ctx: Arc<ContextData>) -> Result<Action, kube::Error> {
    info!("Reconciling Echo resource: {}", echo.name_any());
    let message = &echo.spec.message;
    info!("Observed message payload: '{}'", message);
    Ok(Action::requeue(Duration::from_secs(30)))
}

// Error handler function
fn on_error(echo: Arc<Echo>, error: &kube::Error, _ctx: Arc<ContextData>) -> Action {
    error!("Reconciliation failed for {}: {:?}", echo.name_any(), error);
    Action::requeue(Duration::from_secs(5))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    info!("Starting Kubernetes Operator...");

    let client = Client::try_default().await?;
    let echoes: Api<Echo> = Api::all(client.clone());
    let context = Arc::new(ContextData { client });

    // Start controller using watcher config
    Controller::new(echoes, watcher::Config::default())
        .run(reconcile, on_error, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("Reconciled successfully: {:?}", o),
                Err(e) => error!("Reconciliation error: {:?}", e),
            }
        })
        .await;

    Ok(())
}
