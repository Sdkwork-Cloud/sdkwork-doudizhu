use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[HttpRoute::dual_token(
    HttpMethod::Get,
    "/backend/v3/api/doudizhu/matches",
    "doudizhu",
    "backend.doudizhu.match.list",
)];

pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
