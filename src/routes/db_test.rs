use axum::{response::Html, routing::get, Router};
use sqlx::mysql::MySqlPool;
use crate::rs_template::db_test_template::DbTestTemplate; // Import the template struct
use crate::db::GetRecipe; // Import the Recipe struct
use askama::Template; // Import the Template trait to access render()

pub fn db_test_routes(pool: MySqlPool) -> Router {
    Router::new().route("/", get(move || db_test(pool.clone())))
}

async fn db_test(pool: MySqlPool) -> Html<String> {
    // Query the database for recipes
    let recipes = sqlx::query_as::<_, GetRecipe>("SELECT idrecipes, recipename, recipecategory FROM recipes")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch recipes");

    // Pass the results to the template
    let context = DbTestTemplate { recipes };
    let rendered = context.render().expect("Failed to render DB test template");

    Html(rendered)
}
