use serde::Deserialize;

#[derive(sqlx::FromRow, Debug)]
pub struct GetRecipe {
    pub idrecipes: i32,
    pub recipename: String,
    pub recipecategory: i32,
}

#[derive(Deserialize, Debug)]
pub struct PostRecipe {
    pub recipename: String,
    pub recipecategory: i32,
}