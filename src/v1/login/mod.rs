/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-02 03:30:08
 * @LastEditTime: 2022-07-08 18:08:03
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\login\mod.rs
 */
use axum::{routing::post, Router};

mod dto;
mod handle;

pub fn router() -> Router {
    Router::new().route("/", post(handle::login_handler))
}
