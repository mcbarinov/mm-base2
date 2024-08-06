use axum::response::Html;
use chrono::DateTime;
use minijinja::{Environment, Value};
use serde::de::Error;
use serde::Serialize;

use crate::HtmlResult;

pub fn init_jinja_env() -> Environment<'static> {
    let mut env = Environment::new();
    minijinja_embed::load_templates!(&mut env);
    env.add_filter("none", none_filter);
    env.add_filter("dt", dt_filter);
    env.add_global("confirm", Value::from_safe_string(r#" onclick="return confirm('sure?')" "#.to_string()));
    env
}

fn none_filter(value: Value) -> Value {
    if value.is_undefined() || value.is_none() {
        Value::from("")
    } else {
        value
    }
}

fn dt_filter(value: Value) -> Result<String, minijinja::Error> {
    if value.is_undefined() || value.is_none() {
        Ok("".to_string())
    } else {
        let res = DateTime::parse_from_rfc3339(&value.to_string())
            .map_err(|_| minijinja::Error::custom("dt filter failed to parse datetime"))?
            .to_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        Ok(res)
    }
}

pub fn render_template<S: Serialize>(env: &Environment, template_name: &str, context: S) -> HtmlResult {
    let tmpl = env.get_template(template_name)?;
    let content = tmpl.render(context)?;
    Ok(Html(content))
}
