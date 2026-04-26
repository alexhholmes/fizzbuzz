use axum::extract::Path;
use axum::Json;
use serde::Serialize;
use tracing::instrument;

#[derive(Serialize, utoipa::ToSchema)]
pub struct GetFizzBuzzResponse {
    n: i64,
    result: String,
}

#[utoipa::path(
    get,
    path = "/fizzbuzz/{n}",
    params(("n" = i64, Path, description = "Integer to fizzbuzz")),
    responses((status = 200, body = GetFizzBuzzResponse))
)]
#[instrument]
pub async fn get_fizzbuzz(Path(n): Path<i64>) -> Json<GetFizzBuzzResponse> {
    let result = match (n % 3 == 0, n % 5 == 0) {
        (true, true) => "fizzbuzz".to_string(),
        (true, false) => "fizz".to_string(),
        (false, true) => "buzz".to_string(),
        (false, false) => n.to_string(),
    };
    Json(GetFizzBuzzResponse { n, result })
}
