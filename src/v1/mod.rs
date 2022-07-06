/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-06 16:18:47
 * @LastEditTime: 2022-07-07 02:33:19
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\mod.rs
 */
use axum::{Router, routing::post, extract::extractor_middleware};

mod login;
mod register;



pub fn v1_router() -> Router {
    Router::new()
        .merge(router())
}


fn router() -> Router {
    Router::new()
        .nest("/login", login::router())
        // .merge(without_auth())
}

fn with_auth() -> Router{
    Router::new()
        .merge(login::router())
        // .route_layer(layer)
}

// fn without_auth() -> Router{
//     todo!()
// }