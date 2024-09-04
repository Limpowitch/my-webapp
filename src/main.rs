use axum::{response::Html, routing::get, Router};
use std::fs;

#[tokio::main]
async fn main() {
    // Build our application with two routes
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/world", get(world));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn hello() -> Html<String> {
    // Read the hello.html template
    let hello_template = fs::read_to_string("templates/hello.html")
        .expect("Something went wrong reading the hello.html file");
    Html(hello_template)
}

async fn world() -> Html<String> {
    // Read the world.html template
    let world_template = fs::read_to_string("templates/world.html")
        .expect("Something went wrong reading the world.html file");
    Html(world_template)
}