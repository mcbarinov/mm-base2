use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Jinja(#[from] minijinja::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("app error: {0}")]
    App(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // tracing::error!("AppError: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
