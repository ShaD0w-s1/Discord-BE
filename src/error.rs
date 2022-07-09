/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-04 01:28:12
 * @LastEditTime: 2022-07-09 19:08:19
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\error.rs
 */
use axum::{
    http::{StatusCode},
    Json,
};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] sea_orm::error::DbErr),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),
    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),



    #[error("未找到该用户")]
    UserNotFound,
    #[error("用户名或密码错误")]
    WrongCredentials,
    #[error("错误的密码")]
    WrongPassword,
    #[error("邮箱已存在")]
    DuplicateUserEmail,
    #[error("用户名已存在")]
    DuplicateUserName,
}

/// 应用程序错误类型
#[derive(Debug)]
pub enum AppErrorType {
    Notfound,
    IncorrectLogin,
    Forbidden,
}

pub type MyResult<T> = std::result::Result<T, AppError>;
pub type AppError = (StatusCode, Json<Value>);

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::UserNotFound => StatusCode::UNAUTHORIZED,
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::WrongPassword => StatusCode::UNAUTHORIZED,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
