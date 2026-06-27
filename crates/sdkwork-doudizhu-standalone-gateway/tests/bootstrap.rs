use std::sync::Arc;

#[tokio::test]
async fn memory_repository_mode_builds_match_service() {
    std::env::set_var("DOUDIZHU_REPOSITORY_MODE", "memory");
    let service = sdkwork_doudizhu_standalone_gateway::build_match_service()
        .await
        .expect("memory match service");
    let _ = Arc::as_ptr(&service);
    std::env::remove_var("DOUDIZHU_REPOSITORY_MODE");
}
