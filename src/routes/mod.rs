mod custom_header;
mod many_typed_headers;
mod one_typed_header;

use axum::{routing::get, Router};

use self::{
    custom_header::custom_header, many_typed_headers::many_typed_headers,
    one_typed_header::one_typed_header,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/typed_header", get(one_typed_header))
        .route("/many_typed_headers", get(many_typed_headers))
        .route("/custom_header", get(custom_header))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
