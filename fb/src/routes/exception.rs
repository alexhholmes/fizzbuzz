use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::models::exception::Exception;
use crate::resources::Resources;

#[derive(Debug, Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct ExceptionRequest {
    n: i64,
    result: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ExceptionResponse {
    n: i64,
    result: String,
}

impl From<&Exception> for ExceptionResponse {
    fn from(e: &Exception) -> Self {
        Self {
            n: e.n,
            result: e.result.clone(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/exceptions",
    responses(
        (status = 200, body = Vec<ExceptionResponse>),
        (status = 500)
    )
)]
#[instrument(skip(res))]
pub async fn get_exception_list(
    State(res): State<Resources>,
) -> Result<Json<Vec<ExceptionResponse>>, StatusCode> {
    let exception_list = res
        .db
        .get_exception_list()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        exception_list.iter().map(ExceptionResponse::from).collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/exceptions/{n}",
    params(("n" = i64, Path, description = "Exception key")),
    responses(
        (status = 200, body = ExceptionResponse),
        (status = 404),
        (status = 500)
    )
)]
#[instrument(skip(res))]
pub async fn get_exception(
    Path(n): Path<i64>,
    State(res): State<Resources>,
) -> Result<Json<ExceptionResponse>, StatusCode> {
    let mut cache = res.cache;
    if let Ok(false) = cache.bf_exists("bf:exceptions", &n.to_string()).await {
        return Err(StatusCode::NOT_FOUND);
    }
    if let Some(result) = cache.get(&format!("exception:{n}")).await {
        return Ok(Json(ExceptionResponse { n, result }));
    }
    Err(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    post,
    path = "/exceptions",
    request_body = ExceptionRequest,
    responses(
        (status = 201, body = ExceptionResponse),
        (status = 200, body = ExceptionResponse),
        (status = 500)
    )
)]
#[instrument(skip(res))]
pub async fn post_exception(
    State(res): State<Resources>,
    Json(body): Json<ExceptionRequest>,
) -> Result<(StatusCode, Json<ExceptionResponse>), StatusCode> {
    let (exception, created) = res
        .db
        .upsert_exception(body.n, body.result)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut cache = res.cache;
    if created {
        cache
            .set_with_bf_add("bf:exceptions", &exception.n.to_string(), &exception, 300)
            .await;
    } else {
        cache.set(&exception, 300).await;
    }

    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(ExceptionResponse::from(&exception))))
}
