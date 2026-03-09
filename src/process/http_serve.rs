use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use tracing::{info, warn};

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
    info!("Server normally stopped");

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file_path = std::path::Path::new(&state.path).join(path);
    info!("Reading file: {:?}", file_path);
    if !file_path.exists() {
        warn!("File not found: {:?}", file_path);
        (StatusCode::NOT_FOUND, "File not found".to_string())
    } else {
        // 读取文件内容，但没有考虑文件大小，可能会导致内存溢出
        match tokio::fs::read_to_string(file_path).await {
            Ok(content) => {
                info!("File found, content length: {:?}", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
