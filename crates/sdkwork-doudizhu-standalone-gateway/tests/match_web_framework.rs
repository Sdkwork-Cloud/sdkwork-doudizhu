use axum::body::to_bytes;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_doudizhu_standalone_gateway::{
    build_memory_match_service, with_doudizhu_app_request_context,
};
use sdkwork_routes_match_app_api::build_match_app_router;
use sdkwork_web_core::{access_token_jwt, auth_token_jwt};
use tower::ServiceExt;

static DEV_AUTH_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn dev_tokens() -> (String, String) {
    (
        format!(
            "Bearer {}",
            auth_token_jwt("100001", "user-1", "session-1", "doudizhu")
        ),
        access_token_jwt("100001", "user-1", "session-1", "doudizhu"),
    )
}

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
    let _env_guard = DEV_AUTH_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    std::env::set_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK", "true");
    let (auth_token, access_token) = dev_tokens();
    let router =
        with_doudizhu_app_request_context(build_match_app_router(build_memory_match_service()));

    let response = router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/doudizhu/matches")
                .header("Authorization", auth_token)
                .header("Access-Token", access_token)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    std::env::remove_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK");

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload["code"], 0);
    assert!(payload["data"]["items"].is_array());
    assert_eq!(payload["data"]["pageInfo"]["mode"], "offset");
    assert!(payload["traceId"]
        .as_str()
        .is_some_and(|value| !value.is_empty()));
}

#[tokio::test]
async fn build_router_merges_health_and_match_routes() {
    let _env_guard = DEV_AUTH_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    std::env::set_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK", "true");
    let (auth_token, access_token) = dev_tokens();
    let router = sdkwork_doudizhu_standalone_gateway::build_router(build_memory_match_service());

    for uri in ["/healthz", "/readyz", "/app/v3/api/doudizhu/matches"] {
        let mut builder = Request::builder().uri(uri);
        if uri.contains("/doudizhu/matches") {
            builder = builder
                .header("Authorization", &auth_token)
                .header("Access-Token", &access_token);
        }
        let response = router
            .clone()
            .oneshot(builder.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{uri}");
    }
    std::env::remove_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK");
}
