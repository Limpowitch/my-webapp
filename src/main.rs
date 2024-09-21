mod routes;       // Declare the routes module (from the routes directory)
mod rs_template;  // Declare the templates module (from the rs_template directory)
mod db;           // Declare the db module (from db.rs)

use axum::Router;
use dotenvy::dotenv;
use sqlx::mysql::MySqlPool;
use tower_http::services::fs::ServeDir;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool for MySQL
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Build the Axum application by routing to different parts of the app
    let app = Router::new()
        .nest("/navbar", routes::navbar::navbar_routes())    // Navbar route
        .nest("/homepage", routes::homepage::homepage_routes()) // Homepage route
        .nest("/db_test", routes::db_test::db_test_routes(pool)) // DB test route
        .nest_service("/static", ServeDir::new("static"));   // Serve static files like CSS, JS

    // Run the Axum web app with Hyper on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}