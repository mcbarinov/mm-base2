use crate::jinja::render_template;
use crate::{HtmlResult, JsonResult, RedirectResult};
use axum::response::Redirect;
use axum::Json;
use serde::Serialize;
use serde_json::json;

pub trait Base2State {
    fn templates(&self) -> &minijinja::Environment<'static>;

    fn html(&self, template_name: &str, data: impl Serialize) -> HtmlResult {
        render_template(self.templates(), template_name, data)
    }

    fn json(&self, data: impl Serialize) -> JsonResult {
        Ok(Json(json!(data)))
    }

    fn redirect(&self, path: &str) -> RedirectResult {
        Ok(Redirect::to(path))
    }
}
