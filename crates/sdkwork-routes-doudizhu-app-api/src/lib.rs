mod error;
mod paths;
mod routes;

pub use error::{map_match_error, ok_page_envelope, ok_resource_envelope};
pub use paths::{MATCH_DETAIL_PATH, MATCH_LIST_PATH};
pub use routes::{build_match_app_router, respond_list, MatchListQuery, MatchStore};
