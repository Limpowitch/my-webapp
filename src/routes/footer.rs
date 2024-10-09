use axum::{response::Html, routing::get, Router};
use crate::rs_template::footer_template::FooterTemplate;
use askama::Template;

pub fn footer_routes() -> Router {
    Router::new().route("/", get(footer))
}

async fn footer() -> Html<String> {
    // Create a context for the Navbar template
    let context = FooterTemplate;

    // Render the template to a string
    let rendered = context.render().expect("Failed to render navbar template");

    Html(rendered)
}