use axum::{middleware, Router};
use minijinja::Environment;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_method::api_method_router;
use crate::asset::assets_router;
use crate::auth::{access_token_middleware, auth_router, auth_state};
use crate::system::system_router;

pub fn router_without_state<T: Clone + Sync + Send + 'static>(
    access_token: &str,
    https_schema: bool,
    data_dir: &str,
    jinja_env: Environment<'static>,
    ui_router: Router<T>,
    api_router: Router<T>,
    swagger: SwaggerUi,
) -> Router<T> {
    let auth_state = auth_state(access_token, jinja_env.clone());
    Router::new()
        .merge(swagger)
        .merge(ui_router)
        .nest("/api", api_router)
        .merge(api_method_router(access_token, https_schema))
        .merge(system_router(data_dir))
        .merge(auth_router(access_token, jinja_env))
        .merge(assets_router())
        .layer(middleware::from_fn_with_state(auth_state, access_token_middleware))
}
