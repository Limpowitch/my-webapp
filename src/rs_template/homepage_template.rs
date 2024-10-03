use askama::Template;
use crate::db::GetRecipe; // Import the Recipe struct from the db module

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {
    pub recipes: Vec<GetRecipe>,
}
