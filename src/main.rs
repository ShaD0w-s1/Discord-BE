use std::{net::SocketAddr, io::{self, Read}};
use axum::{
    response::Html,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{ encode };
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categoies")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}



const SECRET_KEY: &[u8] = b"secret";

#[tokio::main]
async fn main() {
    let cfg = intialize_dbconfig();
    let client = get_client(cfg).await.unwrap();

    find_by_username(client).await.unwrap();


    let cors = CorsLayer::permissive();

    let api_nest = Router::new().route("/login", post(login_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api/v1", api_nest)
        .layer(cors)
        .into_make_service();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr).serve(app).await.unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    exp: i64,
    aud: String,
}

async fn login_handler(Json(login_data): Json<LoginRequest>) -> Json<LoginResponse> {
    let header = jsonwebtoken::Header::default();
    let claims = Claims {
        sub: login_data.username,
        iss: "localhost".to_string(),
        exp: 15_876_543_200,
        aud: "localhost".to_string(),
    };

    let key = jsonwebtoken::EncodingKey::from_secret(SECRET_KEY);
    let access_token = encode(&header, &claims, &key).unwrap();
    let refresh_token = encode(&header, &claims, &key).unwrap();
    Json(LoginResponse {
        access_token,
        refresh_token,
    })
}
