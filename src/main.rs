use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "astechlab_card=info,tower_http=info".into()),
        )
        .init();

    let app = Router::new()
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true))
        .layer(TraceLayer::new_for_http());

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("ASTechLab card site listening on http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("server crashed");
}
