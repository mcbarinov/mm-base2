use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Host, Request, State};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use reqwest::Client;

use crate::Error;

pub struct ApiMethodState {
    access_token: String,
    https_schema: bool,
}

impl ApiMethodState {
    pub fn new(access_token: String, https_schema: bool) -> Self {
        Self { access_token, https_schema }
    }
}

async fn api_method(
    Host(hostname): Host,
    State(state): State<Arc<ApiMethodState>>,
    req: Request,
) -> Result<Response<Body>, Error> {
    let mut uri = req.uri().to_string();
    let method = if uri.starts_with("/api-post/") {
        uri = uri.replacen("/api-post/", "/api/", 1);
        Method::POST
    } else if uri.starts_with("/api-delete/") {
        uri = uri.replacen("/api-delete/", "/api/", 1);
        Method::DELETE
    } else {
        panic!("unsupported api method: {}", uri);
    };
    let schema = if state.https_schema { "https" } else { "http" };
    let url = format!("{schema}://{hostname}{uri}");
    let res = Client::new().request(method, url).header("access-token", &state.access_token).send().await?;
    let headers = res.headers().clone();

    let mut response: Response<Body> = Response::builder().body(Body::from_stream(res.bytes_stream())).unwrap();

    response.headers_mut().extend(headers.into_iter().map(|(name, value)| {
        let name = HeaderName::from_bytes(name.unwrap().as_ref()).unwrap();
        let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
        (name, value)
    }));
    Ok(response)
}

pub fn api_method_router<T>(access_token: &str, https_schema: bool) -> Router<T> {
    let state = Arc::new(ApiMethodState::new(access_token.to_string(), https_schema));
    Router::new().route("/api-post/*path", get(api_method)).route("/api-delete/*path", get(api_method)).with_state(state)
}
