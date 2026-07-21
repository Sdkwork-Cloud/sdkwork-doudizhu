use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_game_match_service::{GameError, GameMatchPage};
use sdkwork_utils_rust::{PageInfo, PageMode, SdkWorkApiResponse, SdkWorkPageData};
use sdkwork_web_core::{
    problem_response, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
};
use serde::Serialize;
use serde_json::json;

pub fn map_match_error(error: GameError, ctx: &WebRequestContext) -> Response {
    let kind = match error.code() {
        "not_found" => WebFrameworkErrorKind::NotFound,
        "invalid" => WebFrameworkErrorKind::BadRequest,
        _ => WebFrameworkErrorKind::InternalServerError,
    };
    problem_response(
        &WebFrameworkError {
            kind,
            message: error.message().to_owned(),
            retry_after_seconds: None,
        },
        ctx.problem_correlation(),
    )
}

pub fn ok_resource_envelope<T: Serialize>(ctx: &WebRequestContext, item: T) -> Response {
    Json(SdkWorkApiResponse::success(
        json!({ "item": item }),
        ctx.resolved_trace_id(),
    ))
    .into_response()
}

pub fn ok_page_envelope(ctx: &WebRequestContext, page: GameMatchPage) -> Response {
    let total_pages = page.total.div_ceil(u64::from(page.page_size));
    let data = SdkWorkPageData {
        items: page.items,
        page_info: PageInfo {
            mode: PageMode::Offset,
            page: Some(i32::try_from(page.page).unwrap_or(i32::MAX)),
            page_size: Some(i32::try_from(page.page_size).unwrap_or(i32::MAX)),
            total_items: Some(page.total.to_string()),
            total_pages: Some(i32::try_from(total_pages).unwrap_or(i32::MAX)),
            next_cursor: None,
            has_more: Some(u64::from(page.page) < total_pages),
        },
    };
    Json(SdkWorkApiResponse::success(data, ctx.resolved_trace_id())).into_response()
}
