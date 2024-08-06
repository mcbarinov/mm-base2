use axum::response::Html;
use axum::Json;
use serde_json::Value;

pub use api_method::api_method_router;
pub use api_method::ApiMethodState;
pub use asset::base2_assets_router;
pub use error::Error;
pub use jinja::init_jinja_env;

mod api_method;
mod asset;
mod error;
mod jinja;

pub type JsonResult = Result<Json<Value>, Error>;
pub type HtmlResult = Result<Html<String>, Error>;
