use axum::{response::Html, routing::get, Router};
use sqlx::mysql::MySqlPool;
use crate::rs_template::homepage_template::HomepageTemplate; // Import the template struct
use crate::db::GetRecipe; // Import the Recipe struct
use askama::Template; // Import the Template trait to access render()

pub fn homepage_routes(pool: MySqlPool) -> Router {
    Router::new().route("/", get(move || homepage(pool.clone())))
}

async fn homepage(pool: MySqlPool) -> Html<String> {
    // Query the database for recipes
    let recipes = sqlx::query_as::<_, GetRecipe>("SELECT recipeid, recipename, recipecategory, recipeimageurl FROM recipes ORDER BY RAND() LIMIT 4")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch recipes");

    // Pass the results to the template
    let context = HomepageTemplate { recipes };
    let rendered = context.render().expect("Failed to render DB test template");

    Html(rendered)
}
