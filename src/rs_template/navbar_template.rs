use askama::Template;

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct NavbarTemplate;
