/*
 * @Author: zhangyuxuan
 * @Date: 2022-06-13 22:52:30
 * @LastEditTime: 2022-07-07 02:03:48
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\lib.rs
 */

pub mod error;
pub mod share;
pub mod v1;

pub type MyResult<T> = std::result::Result<T, error::AppError>;

use axum::{extract::Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;


 pub fn api_router() -> Router {
    Router::new()
        .nest("/v1", v1::v1_router())
}