use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{Redirect, Response};
use axum::routing::get;
use axum::{Form, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use cookie::time::Duration;
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};

use crate::jinja::render_template;
use crate::HtmlResult;

pub struct AuthStateInner {
    access_token: String,
    jinja_env: Environment<'static>,
}

pub type AuthState = Arc<AuthStateInner>;

pub fn auth_state(access_token: &str, jinja_env: Environment<'static>) -> AuthState {
    Arc::new(AuthStateInner { access_token: access_token.to_string(), jinja_env })
}

pub fn auth_router<T>(access_token: &str, jinja_env: Environment<'static>) -> Router<T> {
    let state = auth_state(access_token, jinja_env);
    Router::new()
        .route("/auth/login", get(login_page_handler).post(login_handler))
        .route("/auth/logout", get(logout_handler))
        .with_state(state)
}

#[derive(Serialize, Deserialize)]
struct LoginForm {
    access_token: String,
}

async fn login_page_handler(state: State<AuthState>) -> HtmlResult {
    render_template(&state.jinja_env, "login.html", context! {})
}

async fn login_handler(
    State(state): State<AuthState>,
    mut jars: CookieJar,
    Form(form): Form<LoginForm>,
) -> (CookieJar, Redirect) {
    if form.access_token == state.access_token.clone() {
        let cookie =
            Cookie::build(("access-token", state.access_token.clone())).path("/").http_only(true).max_age(Duration::days(30));
        jars = jars.add(cookie);
    }
    (jars, Redirect::to("/"))
}

async fn logout_handler(mut jars: CookieJar) -> (CookieJar, Redirect) {
    let cookie = Cookie::build(("access-token", "")).path("/").http_only(true);
    jars = jars.add(cookie);
    (jars, Redirect::to("/"))
}

pub async fn access_token_middleware(
    State(state): State<AuthState>,
    jar: CookieJar,
    Query(query_params): Query<HashMap<String, String>>,
    request: Request,
    next: Next,
) -> Response {
    let mut auth_ok = false;

    if request.uri() != "/auth/login" {
        // query
        if let Some(value) = query_params.get("access-token") {
            auth_ok = value == &state.access_token;
        }
        // header
        if let Some(value) = request.headers().get("access-token") {
            if let Ok(value) = value.to_str() {
                auth_ok = value == state.access_token;
            }
        }
        // cookie
        if let Some(value) = jar.get("access-token") {
            auth_ok = value.value() == state.access_token;
        }
    } else {
        auth_ok = true;
    }
    if auth_ok {
        next.run(request).await
    } else {
        Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("access denied")).unwrap()
    }
}
