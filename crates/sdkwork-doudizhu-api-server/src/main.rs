use sdkwork_doudizhu_api_server::{build_match_service, build_router};
use sdkwork_utils_rust::optional::default_if_blank;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let bind_address = default_if_blank(
        std::env::var("DOUDIZHU_API_BIND")
            .ok()
            .or_else(|| std::env::var("SDKWORK_DOUDIZHU_APPLICATION_PUBLIC_INGRESS_BIND").ok())
            .as_deref(),
        "127.0.0.1:8096",
    );

    let store = build_match_service()
        .await
        .expect("doudizhu match service bootstrap failed");
    let app = build_router(store);
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind doudizhu api-server listener failed");
    tracing::info!("sdkwork-doudizhu-api-server listening on {bind_address}");
    axum::serve(listener, app)
        .await
        .expect("serve doudizhu api-server failed");
}
