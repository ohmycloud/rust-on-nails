use std::net::SocketAddr;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use grpc_api::api::users_server::UsersServer;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_livereload::LiveReloadLayer;

use crate::api_service::UsersService;

mod api_service;
mod config;
mod errors;
mod root;
mod static_files;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    let grpc_addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    println!("listening on http://{}", addr);
    println!("gRPC server listening on {}", grpc_addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let http_server = axum::serve(listener, app.into_make_service());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(
            "../../grpc-api/generated/api_descriptor_set.bin"
        ))
        .build_v1()
        .unwrap();

    // start gRPC server on a different port
    let grpc_server = Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(reflection_service)
        .add_service(UsersServer::new(UsersService { pool: pool.clone() }))
        .serve(grpc_addr);

    // run our application
    tokio::select! {
        http_result = http_server => {
            http_result?;
        }
        grpc_result = grpc_server => {
            grpc_result?;
        }
    };
    Ok(())
}
