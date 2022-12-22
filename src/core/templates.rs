use anyhow::Result;
use handlebars::Handlebars;
use k8s_openapi::api::core::v1::{Node, Service};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
    pub services: BTreeMap<String, Service>,
    pub nodes: Vec<Node>,
}

/**
 * Render the template with the correct variables.
 */
pub fn render_templates() -> anyhow::Result<(Handlebars<'static>)> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("haproxy", "templates/haproxy.conf")?;
    Ok(handlebars)
}
