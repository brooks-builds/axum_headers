mod simple_common_header;
mod multiple_headers;
mod response_headers;

use axum::{Router, routing::get};

use self::{simple_common_header::simple_common_header, multiple_headers::multiple_headers, response_headers::response_headers};

pub fn create_router() -> Router {
    Router::new()
    .route("/simple_common_header", get(simple_common_header))
    .route("/multiple_headers", get(multiple_headers))
    .route("/response_headers", get(response_headers))
    .layer(tower_http::trace::TraceLayer::new_for_http())
}
