#[derive(sqlx::FromRow, Debug)]
pub struct Recipe {
    pub idrecipes: i32,
    pub recipename: String,
    pub recipecategory: i32,
}