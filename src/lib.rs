use axum::response::{Html, Redirect};
use axum::Json;
use serde_json::Value;

pub use config::Config;
pub use error::{Error, Result};
pub use jinja::init_jinja_env;
pub use log::init_tracing;
pub use router::router_without_state;
pub use state::Base2State;
pub use system::{clean_logfile, get_logfile};

mod api_method;
mod asset;
mod auth;
mod config;
mod error;
mod jinja;
mod log;
mod router;
mod state;
pub mod system;

pub type PlainTextResult = Result<String>;
pub type JsonResult = Result<Json<Value>>;
pub type HtmlResult = Result<Html<String>>;
pub type RedirectResult = Result<Redirect>;
