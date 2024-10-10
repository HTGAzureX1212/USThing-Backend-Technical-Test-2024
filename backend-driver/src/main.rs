//! # Backend Driver
//!
//! This crate contains the main binary that accepts connections and receives HTTP requests.

#![deny(clippy::pedantic)]
#![deny(warnings)]

use std::env;
#[cfg(not(unix))]
use std::future;

use aide::axum::routing::{get, post_with};
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::gen::extract_schemas;
use aide::openapi::{Info, OpenApi};
use aide::scalar::Scalar;
use axum::{Extension, Json};
use backend_utils::route_documentation;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{info, trace};

/// Main entry point of the backend.
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    trace!("loading environment variables");
    dotenvy::dotenv().ok();

    extract_schemas(false);

    // construct router with routes and corresponding handlers
    let router = ApiRouter::new()
        .layer(TraceLayer::new_for_http())
        .route("/docs", Scalar::new("/api.json").axum_route())
        .api_route(
            "/api/v1/example",
            post_with(
                backend_routes::example,
                route_documentation("Example route", "Example documentation"),
            ),
        )
        .route("/api.json", get(openapi_spec));

    // create a TCP listener
    let listener = TcpListener::bind((
        env::var("DOMAIN").unwrap(),
        env::var("PORT").unwrap().parse().unwrap(),
    ))
    .await
    .unwrap();

    // code generation configuration for API docs
    let mut api = OpenApi {
        info: Info {
            title: String::from("USThing Backend Technical Test API"),
            description: Some(String::from(
                "USThing Backend Technical Test API Documentation",
            )),
            version: env!("CARGO_PKG_VERSION").to_string(),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    // start server
    info!("server listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router
            .finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .with_graceful_shutdown(shutdown())
    .await
    .unwrap();
}

/// An internal route returning an open-api specification JSON.
async fn openapi_spec(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

/// Graceful shutdown callback.
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
