use anyhow::Result;
use axum::{
    // extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf};
// use std::sync::Arc;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use tracing::{info, warn, Level};

// /// HTTP 服务器状态结构体
// /// 存储静态文件服务的根目录路径
// #[derive(Debug)]
// struct HttpServerState {
//     path: PathBuf,
// }

/// 自定义 404 错误处理函数
/// 当请求的文件不存在时返回 404 状态码
async fn custom_404_handler() -> (StatusCode, &'static str) {
    warn!("404 Not Found");
    (StatusCode::NOT_FOUND, "File not found")
}

/// 处理 HTTP 服务器启动和运行
///
/// # 参数
/// - `port`: 服务器监听的端口号
/// - `path`: 静态文件服务的根目录路径
///
/// # 返回值
/// - `Result<()>`: 成功启动服务器并正常运行，或返回错误
pub async fn process_http_serve(port: u16, path: PathBuf) -> Result<()> {
    // addr 是一个结构体，包含了 ip 和 port, 这里使用了默认的 ip 127.0.0.1
    // 类型是 SocketAddr，这里使用了 from 方法将 ([127, 0, 0, 1], port) 转换为 SocketAddr
    // socketaddr是一个std::net::SocketAddr 类型，它包含了 ip 和 port 信息
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    // info 日志，记录服务器启动信息
    info!("Serving {:?} on port {}", path, addr);

    // 1. 创建静态文件服务
    // 使用 tower-http 的 ServeDir 提供静态文件服务
    // ServeDir 会自动处理文件的读取和响应
    let static_service = ServeDir::new(&path)
        // 设置 404 错误处理函数
        .not_found_service(get(custom_404_handler));

    // 2. 构建路由并添加中间件
    let router = Router::new()
        // 使用 tower 静态文件服务
        // 将所有请求路由到静态文件服务
        .fallback_service(static_service)
        // 可以添加其他路由，例如 API 路由
        // 示例：添加一个简单的 API 端点
        .route("/api/hello", get(|| async { "Hello, World!" }))
        // 添加 CORS 中间件
        // 允许跨域请求，方便前端应用访问
        .layer(CorsLayer::permissive())
        // 添加压缩中间件
        // 减少网络传输量，提高响应速度
        .layer(CompressionLayer::new())
        // 添加跟踪中间件
        // 记录请求和响应的详细信息，便于调试和监控
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    // 3. 启动服务器
    // 创建 TCP 监听器，绑定到指定地址
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server started on {:?}", addr);
    // 启动 axum 服务器，处理传入的请求
    axum::serve(listener, router).await?;
    info!("Server normally stopped");
    Ok(())
}

// /// 旧的文件处理函数（保留用于参考）
// ///
// /// 手动处理文件读取和响应
// /// 注意：此函数目前未被使用，已被 tower-http 的 ServeDir 替代
// async fn file_handler(
//     State(state): State<Arc<HttpServerState>>,
//     Path(path): Path<String>,
// ) -> (StatusCode, String) {
//     let file_path = std::path::Path::new(&state.path).join(path);
//     info!("Reading file: {:?}", file_path);
//     if !file_path.exists() {
//         warn!("File not found: {:?}", file_path);
//         (StatusCode::NOT_FOUND, "File not found".to_string())
//     } else {
//         // 读取文件内容，但没有考虑文件大小，可能会导致内存溢出
//         match tokio::fs::read_to_string(file_path).await {
//             Ok(content) => {
//                 info!("File found, content length: {:?}", content.len());
//                 (StatusCode::OK, content)
//             }
//             Err(e) => {
//                 warn!("Error reading file: {:?}", e);
//                 (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
//             }
//         }
//     }
// }
