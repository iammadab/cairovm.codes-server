use serde::Serialize;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use cairo1_run::{Error, CAIRO_LANG_COMPILER_VERSION};

#[derive(Serialize)]
/// Represents the error log type
enum ErrorType {
    Error,
    Warn,
    Info,
}

#[derive(Serialize)]
/// Self contained error entry
struct ErrorEntry {
    error_type: ErrorType,
    message: String,
}

impl ErrorEntry {
    fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
        }
    }
}

impl Default for ErrorEntry {
    fn default() -> Self {
        Self {
            error_type: ErrorType::Error,
            message: "failed to compile and run cairo program".to_string(),
        }
    }
}

#[derive(Serialize)]
// TODO: possibly, rename
//  add documentation
pub(crate) struct ResponseError {
    #[serde(skip)]
    status_code: StatusCode,
    errors: Vec<ErrorEntry>,
    cairo_lang_compiler_version: String,
}

impl ResponseError {
    // TODO: add documentation
    fn new(errors: Vec<ErrorEntry>) -> Self {
        Self {
            status_code: StatusCode::EXPECTATION_FAILED,
            errors,
            cairo_lang_compiler_version: CAIRO_LANG_COMPILER_VERSION.to_string(),
        }
    }

    // TODO: add documentation
    pub(crate) fn get_error(error: Error) -> Self {
        match error {
            Error::DiagnosticsError(diagnostics) => build_diagnostics_response_error(diagnostics),
            _ => ResponseError::new(vec![ErrorEntry::default()]),
        }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

// TODO: add documentation
fn build_diagnostics_response_error(diagnostics: Vec<String>) -> ResponseError {
    let diagnostics_errors = diagnostics.into_iter().map(|message| {
        let error_type = if message.starts_with("error") {
            ErrorType::Error
        } else {
            ErrorType::Warn
        };
        ErrorEntry::new(error_type, message)
    });
    ResponseError::new(diagnostics_errors.collect())
}
