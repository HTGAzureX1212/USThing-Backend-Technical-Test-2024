use aide::OperationIo;
use axum::response::{IntoResponse, Response};
use axum_jsonschema::JsonSchemaRejection;
use axum_macros::FromRequest;
use serde::Serialize;
use serde_json::json;

#[derive(FromRequest, OperationIo)]
#[from_request(via(axum_jsonschema::Json), rejection(DummyError))]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub enum DummyError {
    Dummy,
}

impl IntoResponse for DummyError {
    fn into_response(self) -> Response {
        Json(json!({
            "code": 500,
            "status": "Internal server error",
        }))
        .into_response()
    }
}

impl From<JsonSchemaRejection> for DummyError {
    fn from(rejection: JsonSchemaRejection) -> Self {
        match rejection {
            JsonSchemaRejection::Json(_) => Self::Dummy,
            JsonSchemaRejection::Serde(_) => Self::Dummy,
            JsonSchemaRejection::Schema(_) => Self::Dummy,
        }
    }
}
