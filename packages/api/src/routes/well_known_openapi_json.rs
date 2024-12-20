use super::api;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(api::post, api::get))]
pub struct OpenApiSchema;

pub async fn get() -> Response {
    Json(OpenApiSchema::openapi()).into_response()
}
