use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::info;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}
// 实现一个简单的http服务器，用于提供静态文件服务
pub async fn process_http_serve(port: u16, path: PathBuf) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving {:?} on port {}", path, addr);
    let state = HttpServerState { path };
    let router = Router::new()
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    info!("Server stopped");

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file_path = std::path::Path::new(&state.path).join(path);
    info!("Reading file: {:?}", file_path);
    if !file_path.exists() {
        (StatusCode::NOT_FOUND, "File not found".to_string())
    } else {
        match tokio::fs::read_to_string(file_path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}
