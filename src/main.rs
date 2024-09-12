use axum::{response::Html, routing::get, routing::post, Router, extract::Json, response::IntoResponse};
use std::fs;
use std::env;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use sqlx::mysql::MySqlPool;
use dotenvy::dotenv;
use tower_http::services::fs::ServeDir;
use serde::Deserialize;


// Define a global variable for storing the category
static CATEGORY: once_cell::sync::Lazy<Arc<Mutex<Option<String>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

#[tokio::main]
async fn main() {
    //load enviroment variables from .env file
    dotenv().ok();

    //get the database URL from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool for MySQL
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to create pool");

    // Build our application with two routes
    let app = Router::new()
        .route("/homepage", get(homepage))
        .route("/set_category", post(set_category))
        .route("/categories", get(categories))
        .route("/navbar", get(navbar)) 
        .nest_service("/static", ServeDir::new("static")); 

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn homepage() -> Html<String> {
    // Read the hello.html template
    let homepage_template = fs::read_to_string("templates/homepage.html")
        .expect("Something went wrong reading the homepage.html file");
    Html(homepage_template)
}

#[derive(Deserialize)]
struct CategoryPayload{
    category: String,
}

async fn set_category(
    Json(payload): Json<CategoryPayload>,
) -> impl IntoResponse {
    let mut category = CATEGORY.lock().unwrap();
    *category = Some(payload.category);
    "Category received".into_response()
}

async fn categories() -> Html<String> {
    let category = CATEGORY.lock().unwrap();
    let category_display = match &*category {
        Some(cat) => format!("Selected Category: {}", cat),
        None => "No category selected".to_string(),
    };

    let categories_template = fs::read_to_string("templates/categories.html")
        .expect("Something went wrong reading the categories.html file");

    Html(format!("{}<p>{}</p>", categories_template, category_display))
}

async fn navbar() -> Html<String> {
    // Read the navbar.html template
    let navbar_template = fs::read_to_string("templates/navbar.html")
        .expect("Something went wrong reading the navbar.html file");
    Html(navbar_template)
}