/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-06 16:18:47
 * @LastEditTime: 2022-07-09 20:51:01
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\mod.rs
 */
use axum::Router;

mod login;
mod register;

pub fn v1_router() -> Router {
    let without_auth = without_auth();
    let with_auth = with_auth();
    Router::new().merge(without_auth).merge(with_auth)
}

fn with_auth() -> Router {
    Router::new()
}

fn without_auth() -> Router {
    Router::new()
        .nest("/register", register::router())
        .nest("/login", login::router())
}
