pub mod bootstrap;
mod web_bootstrap;

pub use bootstrap::{
    build_match_service, build_memory_match_service, build_router, SharedMatchService,
};
pub use route_manifest::{DOUDIZHU_APP_HTTP_ROUTES, DOUDIZHU_BACKEND_HTTP_ROUTES};
pub use web_bootstrap::{
    doudizhu_public_path_prefixes, with_doudizhu_app_request_context,
    with_doudizhu_backend_request_context,
};

pub mod route_manifest {
    include!(concat!(env!("OUT_DIR"), "/doudizhu_http_routes.rs"));
}
