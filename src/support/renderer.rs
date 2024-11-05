use askama::Template;
use axum::response::Html;


pub fn render<T: Template>(template: T) -> Html<String> {
    Html( template.render().unwrap_or_else(|_| "".into()))
}
