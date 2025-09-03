use rmcp::transport::{
    StreamableHttpServerConfig, StreamableHttpService,
    streamable_http_server::session::local::LocalSessionManager,
};
use std::sync::Arc;

pub async fn init_router() -> axum::Router {
    let service = StreamableHttpService::new(
        || Ok(crate::counter::Counter::new()),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);

    router
}
