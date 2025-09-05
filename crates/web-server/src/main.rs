use std::net::SocketAddr;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use grpc_api::api::users_server::UsersServer;
use tonic::transport::Server;
use tower_livereload::LiveReloadLayer;

use crate::api_service::UsersService;

mod api_service;
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
        .route("/new_user", post(root::new_user_action))
        .route("/static/{*path}", get(static_files::static_path))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // Start gRPC server on a different port
    let grpc_addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    let grpc_server = Server::builder()
        .add_service(UsersServer::new(UsersService { pool: pool.clone() }))
        .serve(grpc_addr);

    // run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("listening on http://{}", addr);
    println!("gRPC server listening on {}", grpc_addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tokio::select! {
        http_result = axum::serve(listener, app.into_make_service()) => {
            http_result.unwrap();
        }
        grpc_result = grpc_server => {
            grpc_result.unwrap();
        }
    };
}
