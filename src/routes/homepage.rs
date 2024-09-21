use axum::{response::Html, routing::get, Router};
use crate::rs_template::homepage_template::HomepageTemplate;
use askama::Template;

pub fn homepage_routes() -> Router {
    Router::new().route("/", get(homepage))
}

async fn homepage() -> Html<String> {
    // Create a context for the template with a name value
    let context = HomepageTemplate { name: "World" };

    // Render the template to a string
    let rendered = context.render().expect("Failed to render template");

    Html(rendered)
}