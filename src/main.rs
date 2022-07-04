/*
 * @Author: zhangyuxuan
 * @Date: 2022-06-03 20:45:34
 * @LastEditTime: 2022-07-04 22:58:26
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\main.rs
 */
use axum::{
    extract::Extension,
    response::Html,
    routing::{get, post},
    Json, Router,
};

use migration::{Migrator, MigrationTrait, MigratorTrait};

use jsonwebtoken::encode;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

const SECRET_KEY: &[u8] = b"secret";

async fn database() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db: DatabaseConnection =
        Database::connect("PostgreSQL://postgres:zzs852329@192.168.1.5/postgres").await?;
    Ok(db)
}

use sea_orm::tests_cfg::*;


fn tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}

#[tokio::main]
async fn main() {
    tracing();
    
    let connect = Database::connect("PostgreSQL://postgres:zzs852329@192.168.1.5/postgres").await.unwrap();

    Migrator::up(&connect, None);

    let builder = connect.get_database_backend();
    let schema = Schema::new(builder);

    builder.build(&schema.create_table_from_entity(CakeFillingPrice));

    

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


// Json<LoginResponse>
async fn login_handler(
    Json(login_data): Json<LoginRequest>,
    Extension(db): Extension<DatabaseConnection>,
) -> () {

    let name = entity::user::Entity::find()
        .filter(entity::user::Column::Name);

    match name {
        Some(name) => {
            println!("{:?}", name);
        }
        None => {
            println!("None");
        }
    }
        
  

    
    // let header = jsonwebtoken::Header::default();
    // let claims = Claims {
    //     sub: login_data.username,
    //     iss: "localhost".to_string(),
    //     exp: 15_876_543_200,
    //     aud: "localhost".to_string(),
    // };
    // let key = jsonwebtoken::EncodingKey::from_secret(SECRET_KEY);
    // let access_token = encode(&header, &claims, &key).unwrap();
    // let refresh_token = encode(&header, &claims, &key).unwrap();
    // Json(LoginResponse {
    //     access_token,
    //     refresh_token,
    // })
}
