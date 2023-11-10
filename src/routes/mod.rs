mod custom_header;
mod custom_header_response;
mod many_typed_headers;
mod one_typed_header;
mod simple_header_response;

use axum::{routing::get, Router};

use self::{
    custom_header::custom_header, custom_header_response::custom_header_response,
    many_typed_headers::many_typed_headers, one_typed_header::one_typed_header,
    simple_header_response::simple_header_response,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/typed_header", get(one_typed_header))
        .route("/many_typed_headers", get(many_typed_headers))
        .route("/custom_header", get(custom_header))
        .route("/simple_header_response", get(simple_header_response))
        .route("/custom_header_response", get(custom_header_response))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
