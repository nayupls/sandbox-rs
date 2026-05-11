mod api;
mod config;
mod error;
mod models;
mod openapi;
mod runtimes;
mod sandbox;

use std::sync::Arc;

use axum::{routing::get, routing::post, Router};
use config::Config;
use sandbox::docker::DockerSandbox;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub sandbox: Arc<DockerSandbox>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sandbox_rs=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(Config::from_env()?);
    let sandbox = Arc::new(DockerSandbox::new(config.clone()));
    let state = AppState {
        config: config.clone(),
        sandbox,
    };

    let app = Router::new()
        .route("/health", get(api::health))
        .route("/v1/languages", get(api::languages))
        .route("/v1/execute", post(api::execute))
        .merge(SwaggerUi::new("/docs").url("/openapi.json", openapi::ApiDoc::openapi()))
        .with_state(state);

    let listener = TcpListener::bind(config.bind_addr).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
