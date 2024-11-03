use serde::Deserialize;

#[derive(sqlx::FromRow, Debug)]
pub struct GetRecipe {
    pub recipeid: i32,
    pub recipename: String,
    pub recipecategory: String,
    pub recipeimageurl: String,
    pub recipedescription: String,
    pub recipetime: i32,
    pub recipeportions: i32,
    pub recipecost: i32,
    pub recipekcal: i32,
}

#[derive(Deserialize, Debug)]
pub struct PostRecipe {
    pub recipename: String,
    pub recipecategory: String,
}