/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-02 03:30:08
 * @LastEditTime: 2022-07-09 01:30:38
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\register\mod.rs
 */

use axum::{Router, routing::post};

mod dto;
mod handle;

pub fn router() -> Router {
    Router::new().route("/", post(handle::register_handler))
}
