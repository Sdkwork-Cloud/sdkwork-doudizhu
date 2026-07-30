use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/doudizhu/matches",
        "doudizhu",
        "doudizhu.match.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/doudizhu/matches/{matchId}",
        "doudizhu",
        "doudizhu.match.retrieve",
    ),
];

pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
