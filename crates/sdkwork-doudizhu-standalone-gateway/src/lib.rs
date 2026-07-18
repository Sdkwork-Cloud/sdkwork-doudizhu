pub mod bootstrap;

pub use bootstrap::{
    build_match_service, build_memory_match_service, build_router, build_router_from_business,
    SharedMatchService,
};
pub use sdkwork_doudizhu_gateway_assembly::route_manifest::{
    DOUDIZHU_APP_HTTP_ROUTES, DOUDIZHU_BACKEND_HTTP_ROUTES,
};
pub use sdkwork_doudizhu_gateway_assembly::{
    doudizhu_public_path_prefixes, with_doudizhu_app_request_context,
    with_doudizhu_backend_request_context,
};
