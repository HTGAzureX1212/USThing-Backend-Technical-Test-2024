use aide::axum::IntoApiResponse;

/// Template route
pub async fn example() -> impl IntoApiResponse {
    "hello world"
}
