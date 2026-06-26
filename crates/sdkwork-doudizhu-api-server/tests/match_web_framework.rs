use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_doudizhu_api_server::{build_memory_match_service, with_doudizhu_app_request_context};
use sdkwork_routes_match_app_api::build_match_app_router;
use tower::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=demo-tenant;user_id=user-1;session_id=session-1;app_id=doudizhu;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=demo-tenant;user_id=user-1;session_id=session-1;app_id=doudizhu;environment=dev;deployment_mode=saas";

#[tokio::test]
async fn match_router_rejects_unauthenticated_requests() {
    let router =
        with_doudizhu_app_request_context(build_match_app_router(build_memory_match_service()));

    let response = router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/doudizhu/matches")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn match_router_accepts_dev_inline_dual_tokens() {
    let router =
        with_doudizhu_app_request_context(build_match_app_router(build_memory_match_service()));

    let response = router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/doudizhu/matches")
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn build_router_merges_health_and_match_routes() {
    let router = sdkwork_doudizhu_api_server::build_router(build_memory_match_service());

    for uri in [
        "/app/v3/api/system/health",
        "/app/v3/api/system/ready",
        "/app/v3/api/doudizhu/matches",
    ] {
        let mut builder = Request::builder().uri(uri);
        if uri.contains("/doudizhu/matches") {
            builder = builder
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN);
        }
        let response = router
            .clone()
            .oneshot(builder.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{uri}");
    }
}
