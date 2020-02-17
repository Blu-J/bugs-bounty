use ::log::error;
use anyhow::Error;
use rweb::http::StatusCode;
use rweb::*;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    status: String,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let status_code;
    let error_message = if err.is_not_found() {
        status_code = StatusCode::NOT_FOUND;
        ErrorMessage {
            status: format!("{}", status_code),
            message: "404".into(),
        }
    } else if let Some(AnyhowError(err)) = err.find() {
        error!("Ran into error: {:?}", err);

        status_code = StatusCode::BAD_REQUEST;

        ErrorMessage {
            status: format!("{}", status_code),
            message: format!("{}", err),
        }
    } else if let Some(_) = err.find::<rweb::reject::MethodNotAllowed>() {
        status_code = StatusCode::METHOD_NOT_ALLOWED;
        ErrorMessage {
            status: format!("{}", status_code),
            message: "404".into(),
        }
    } else {
        error!("unhandled rejection: {:?}", err);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
        // We should have expected this... Just l
        ErrorMessage {
            status: format!("{}", status_code),
            message: "Unhandled rejection".into(),
        }
    };

    Ok(rweb::reply::with_status(
        rweb::reply::json(&error_message),
        status_code,
    ))
}

#[derive(Debug)]
pub struct AnyhowError(pub Error);

impl reject::Reject for AnyhowError {}

impl From<Error> for AnyhowError {
    fn from(value: Error) -> Self {
        AnyhowError(value)
    }
}
