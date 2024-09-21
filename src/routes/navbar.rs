use axum::{response::Html, routing::get, Router};
use crate::rs_template::navbar_template::NavbarTemplate;
use askama::Template;

pub fn navbar_routes() -> Router {
    Router::new().route("/", get(navbar))
}

async fn navbar() -> Html<String> {
    // Create a context for the Navbar template
    let context = NavbarTemplate;

    // Render the template to a string
    let rendered = context.render().expect("Failed to render navbar template");

    Html(rendered)
}
