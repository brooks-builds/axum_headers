use axum::{headers::UserAgent, TypedHeader};

pub async fn one_typed_header(TypedHeader(user_agent): TypedHeader<UserAgent>) {
    tracing::info!("User agent is: {user_agent}");
}
