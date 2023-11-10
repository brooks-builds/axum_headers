use axum::response::IntoResponse;

pub async fn custom_header_response() -> impl IntoResponse {
    (
        [("hello", "hello from axum"), ("message", "something")],
        "message",
    )
}
