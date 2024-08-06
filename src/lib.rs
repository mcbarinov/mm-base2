use axum::response::Html;
use axum::Json;
use serde_json::Value;

pub use api_method::api_method_router;
pub use api_method::ApiMethodState;
pub use asset::assets_router;
pub use auth::{access_token_middleware, auth_router, auth_state};
pub use error::Error;
pub use jinja::init_jinja_env;
pub use system::{clean_logfile, get_logfile, system_router};

mod api_method;
mod asset;
mod auth;
mod error;
mod jinja;
pub mod system;

pub type JsonResult = Result<Json<Value>, Error>;
pub type HtmlResult = Result<Html<String>, Error>;
