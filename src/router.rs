use anyhow::Context;
use axum::{routing, Router};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::html::{homepage, about};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();
    info!("Cargando rutas...");

    let src_path = std::env::current_dir().unwrap();


    let port = 6900_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        .route("/", routing::get(homepage))
        .route("/about", routing::get(about))
        .nest_service(
            "/styles",
            ServeDir::new(format!("{}/public/styles", src_path.to_str().unwrap())),
        );

    info!("URL del servicio: {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("Servicio fallo")
        .expect("Servicio fallo");
    Ok(())
}
