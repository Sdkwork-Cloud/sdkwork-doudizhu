use sdkwork_api_doudizhu_assembly::assemble_api_router;
use sdkwork_api_doudizhu_standalone_gateway::build_router_from_business;
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

    let assembly = assemble_api_router()
        .await
        .expect("doudizhu gateway assembly failed");
    let app = build_router_from_business(assembly.router);
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind doudizhu standalone-gateway listener failed");
    tracing::info!("sdkwork-api-doudizhu-standalone-gateway listening on {bind_address}");
    axum::serve(listener, app)
        .await
        .expect("serve doudizhu standalone-gateway failed");
}
