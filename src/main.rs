use axum::{response::Html, routing::get, Router};
use std::fs;
use sqlx::mysql::MySqlPool;
use std::env;
use dotenvy::dotenv;
use tower_http::services::ServeDir;

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
        .route("/hello", get(hello))
        .route("/world", get(world))
        .route("/db_test", get(move || db_test(pool.clone()))) // Route to test the DB connection
        .nest_service("/static", ServeDir::new("static")); 

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

// Function to test database connection
async fn db_test(pool: MySqlPool) -> Html<String> {
    // Try to acquire a connection from the pool
    match pool.acquire().await {
        Ok(_) => Html("<h1>Database Connection Successful!</h1>".to_string()),
        Err(_) => Html("<h1>Failed to Connect to Database</h1>".to_string()),
    }
}