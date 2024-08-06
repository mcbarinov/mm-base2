use std::sync::Arc;

use axum::extract::State;
use axum::routing::{delete, get};
use axum::Router;
use tokio::fs::read_to_string;

#[derive(Debug)]
pub struct SystemState {
    data_dir: String,
}

#[utoipa::path(get, path = "/api/system/log", tag = "system")]
pub async fn get_logfile(state: State<Arc<SystemState>>) -> Result<String, crate::Error> {
    let logfile = format!("{}/app.log", state.data_dir);
    Ok(read_to_string(logfile).await?)
}

#[utoipa::path(delete, path = "/api/system/log", tag = "system")]
pub async fn clean_logfile(state: State<Arc<SystemState>>) -> Result<String, crate::Error> {
    let logfile = format!("{}/app.log", state.data_dir);
    tokio::fs::write(logfile, "").await?;
    Ok("ok".to_string())
}

pub fn system_router<T>(data_dir: &str) -> Router<T> {
    let state = Arc::new(SystemState { data_dir: data_dir.to_string() });
    Router::new().route("/api/system/log", get(get_logfile)).route("/api/system/log", delete(clean_logfile)).with_state(state)
}
