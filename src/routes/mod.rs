use axum::{response::Html, routing::{get, post}, Router};
use axum_embed::ServeEmbed;
use book::get_users;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;
mod book;

#[derive(RustEmbed, Clone)]
#[folder = "public"]
struct Assets;

pub async fn init_routes() {
    let assets = ServeEmbed::<Assets>::new();

    let book = Router::new()
    .route("/books", get(get_users), post());

    let app = Router::new()
        .nest_service("/public", assets)
        .route("/", get(load_home))
        .merge(book);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

pub async fn load_home() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}
