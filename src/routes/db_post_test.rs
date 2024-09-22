use axum::{routing::get, Router, extract::Form, response::Html, response::Redirect};
use sqlx::mysql::MySqlPool;
use crate::db::PostRecipe;
use crate::rs_template::db_post_test_template::DbPostTestTemplate;
use askama::Template;

pub fn db_post_test_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(db_post_test_form).post(move |form: Form<PostRecipe>| db_post_test(pool.clone(), form)))
}

// Handle GET requests: Render the form
async fn db_post_test_form() -> Html<String> {
    let context = DbPostTestTemplate;  // Render the form
    let rendered = context.render().expect("Failed to render form");
    Html(rendered)
}

// Handle POST requests: Insert form data into the database
async fn db_post_test(pool: MySqlPool, form: Form<PostRecipe>) -> Redirect {
    // Insert the new recipe into the database, letting MySQL handle the idrecipes auto-increment
    sqlx::query!(
        "INSERT INTO recipes (recipename, recipecategory) VALUES (?, ?)",
        form.recipename,
        form.recipecategory
    )
    .execute(&pool)
    .await
    .expect("Failed to insert recipe");

    // Redirect back to the form or another page
    Redirect::to("/db_post_test")
}

