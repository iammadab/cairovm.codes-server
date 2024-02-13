// TODO: add crate wide documentation
// TODO: can there be something more general than error?

use serde::Serialize;

use axum::http::StatusCode;
use cairo1_run::{CAIRO_LANG_COMPILER_VERSION, Error};

#[derive(Serialize)]
// TODO: add documentation
enum ErrorType {
    Error,
    Warn,
    Info
}

#[derive(Serialize)]
// TODO: add documentation
struct ErrorEntry {
    error_type: ErrorType,
    message: String
}

impl Default for ErrorEntry {
    fn default() -> Self {
        Self {
            error_type: ErrorType::Error,
            message: "failed to compile and run cairo program".to_string()
        }
    }
}

#[derive(Serialize)]
// TODO: possibly, rename
//  add documentation
struct ResponseError {
    #[serde(skip)]
    status_code: StatusCode,
    errors: Vec<ErrorEntry>,
    cairo_lang_compiler_version: String
}

// TODO: add documentation
//   make this more specific
fn convert_error_to_response_error(error: Error) -> ResponseError {
    ResponseError {
        status_code: StatusCode::EXPECTATION_FAILED,
        errors: vec![ErrorEntry::default()],
        cairo_lang_compiler_version: CAIRO_LANG_COMPILER_VERSION.to_string()
    }
}


