use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use tower_livereload::LiveReloadLayer;

mod config;
mod errors;
mod root;
mod static_files;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader))
        .route("/static/{*path}", get(static_files::static_path))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run our application
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
