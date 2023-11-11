use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "home.html")]
struct HomepageTemplate;

pub async fn homepage() -> impl IntoResponse {
    HtmlTemplate(HomepageTemplate {})
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

pub async fn about() -> impl IntoResponse {
    HtmlTemplate(AboutTemplate {})
}

pub struct HtmlTemplate<T>(T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),

            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed To Template HTML: {}", e),
            )
                .into_response(),
        }
    }
}
