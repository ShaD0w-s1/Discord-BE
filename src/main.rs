use std::net::SocketAddr;

use axum::{
    http::{HeaderValue,Method},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .layer(layer_cors())
        .route("/", get(handler))
        .route("/api/v1/login", post(loginHandler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{}", addr);
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn layer_cors() -> CorsLayer{
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn loginHandler() -> Json<&'static str> {
    Json("<h1>Hello, World!</h1>")
}
