/*
 * @Author: zhangyuxuan
 * @Date: 2022-06-03 20:45:34
 * @LastEditTime: 2022-07-09 21:04:04
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\main.rs
 */



use std::{net::SocketAddr};

use axum::{Server, Router, extract::Extension, routing::get};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::CorsLayer;


use discord_be::{api_router, };

async fn database() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db: DatabaseConnection =
        Database::connect("PostgreSQL://postgres:zzs852329@192.168.1.5/postgres").await?;
    Ok(db)
}


fn tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}

#[tokio::main]
async fn main() {

    tracing();

    let db_connect = Database::connect("PostgreSQL://postgres:zzs852329@localhost/postgres")
        .await
        .expect("数据库连接失败");

    Migrator::up(&db_connect, None);

    let app = Router::new()
        .route("/", get(index_handle))
        .nest("/api",  api_router())
        .layer(Extension(db_connect))
        .layer(CorsLayer::permissive())
        .into_make_service();

    //  数据库迁移
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&addr)
        .serve(app)
        .await
        .expect("服务器启动失败");
}

async fn index_handle() -> String {
    "Hello, world!".to_string()
}