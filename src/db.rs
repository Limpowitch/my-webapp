use serde::Deserialize;

#[derive(sqlx::FromRow, Debug)]
pub struct GetRecipe {
    pub recipeid: i32,
    pub recipename: String,
    pub recipecategory: i32,
    pub recipeimageurl: String,
}

#[derive(Deserialize, Debug)]
pub struct PostRecipe {
    pub recipename: String,
    pub recipecategory: i32,
}