/*
 * @Author: zhangyuxuan
 * @Date: 2022-06-13 22:52:30
 * @LastEditTime: 2022-07-07 22:55:41
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\lib.rs
 */

pub mod error;
pub mod share;
pub mod v1;

use axum::Router;

pub fn api_router() -> Router {
    Router::new().nest("/v1", v1::v1_router())
}
