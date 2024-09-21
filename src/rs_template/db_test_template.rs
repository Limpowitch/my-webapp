use askama::Template;
use crate::db::Recipe; // Import the Recipe struct from the db module

#[derive(Template)]
#[template(path = "db_test.html")]
pub struct DbTestTemplate {
    pub recipes: Vec<Recipe>,
}
