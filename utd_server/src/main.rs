use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::
    net::SocketAddr
;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
    ),
    components(
        schemas() 
    ),
    tags(
        (name = "Management", description = "Operational endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "INFO".into())
                // .unwrap_or_else(|_| "axum_swagger_simple=debug,tower_http=debug,RUST_LOG=INFO".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Server listening on {}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/swagger-ui", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "Management" 
)]
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

