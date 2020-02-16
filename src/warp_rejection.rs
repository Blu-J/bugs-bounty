use ::log::error;
use anyhow::Error;
use rweb::http::StatusCode;
use rweb::*;
use std::convert::Infallible;

#[derive(Debug)]
struct ErrorMessage {
    code: StatusCode,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let error_message = if err.is_not_found() {
        ErrorMessage {
            code: StatusCode::NOT_FOUND,
            message: "Not found".into(),
        }
    } else if let Some(AnyhowError(err)) = err.find() {
        error!("Ran into error: {:?}", err);

        ErrorMessage {
            code: StatusCode::BAD_REQUEST,
            message: "Server error".into(),
        }
    } else if let Some(_) = err.find::<rweb::reject::MethodNotAllowed>() {
        ErrorMessage {
            code: StatusCode::METHOD_NOT_ALLOWED,
            message: "Method not allowed".into(),
        }
    } else {
        error!("unhandled rejection: {:?}", err);
        // We should have expected this... Just l
        ErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Unhandled rejection".into(),
        }
    };

    Ok(rweb::reply::with_status(
        error_message.message,
        error_message.code,
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
