use askama::Template;

#[derive(Template)]
#[template(path = "db_post_test.html")]
pub struct DbPostTestTemplate;