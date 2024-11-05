use app::{modules::logging::init_logging, routes::init_routes};

#[tokio::main]
async fn main() {
    init_logging();
    init_routes().await;
}
