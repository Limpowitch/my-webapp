use askama::Template;

#[derive(Template)]
#[template(path = "footer.html")]
pub struct FooterTemplate;