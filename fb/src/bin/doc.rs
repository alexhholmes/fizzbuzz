//! Generates the OpenAPI spec for this API and prints it to stdout as JSON.
//!
//! utoipa builds the spec at runtime from proc-macro annotations — there is no
//! static codegen step. Run via `make gen-api` or directly:
//!
//! ```
//! cargo run --bin doc > api/openapi.json
//! ```

use fb::routes::{exception, fizzbuzz};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        fizzbuzz::get_fizzbuzz,
        exception::get_exception_list,
        exception::get_exception,
        exception::post_exception,
    ),
    components(schemas(
        fizzbuzz::GetFizzBuzzResponse,
        exception::ExceptionResponse,
        exception::ExceptionRequest,
    ))
)]
struct ApiDoc;

fn main() {
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
}
