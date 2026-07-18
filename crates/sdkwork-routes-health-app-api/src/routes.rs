use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use sdkwork_web_core::WebRequestContext;
use serde_json::json;

pub const HEALTH_PATH: &str = "/healthz";
pub const READY_PATH: &str = "/readyz";

pub fn build_health_router() -> Router {
    Router::new()
        .route(HEALTH_PATH, get(health_check))
        .route(READY_PATH, get(ready_check))
}

async fn health_check(_ctx: WebRequestContext) -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "sdkwork-doudizhu"
        })),
    )
        .into_response()
}

async fn ready_check(_ctx: WebRequestContext) -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ready",
            "service": "sdkwork-doudizhu"
        })),
    )
        .into_response()
}
