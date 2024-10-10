use aide::axum::IntoApiResponse;

pub mod library;

/// Template route
pub async fn example() -> impl IntoApiResponse {
    "hello world"
}
