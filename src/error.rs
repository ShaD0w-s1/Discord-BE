/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-04 01:28:12
 * @LastEditTime: 2022-07-06 16:43:56
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\error.rs
 */
use axum::{
  http::{header, HeaderMap, StatusCode},
  response::IntoResponse,
};

/// 应用程序错误类型
#[derive(Debug)]
pub enum AppErrorType {
  Db,
  Template,
  Notfound,
  Duplicate,
  Crypt,
  IncorrectLogin,
  Forbidden,
}

/// 应用程序错误
#[derive(Debug)]
pub struct AppError {
  pub message: Option<String>,
  pub cause: Option<Box<dyn std::error::Error>>,
  pub types: AppErrorType,
}

impl AppError {
  fn new(
      message: Option<String>,
      cause: Option<Box<dyn std::error::Error>>,
      types: AppErrorType,
  ) -> Self {
      Self {
          message,
          cause,
          types,
      }
  }
  pub fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
      Self::new(None, Some(cause), types)
  }

//  pub fn from_dberr() -> Self {
//     Self::new(message, cause, types)
//  }
  
  pub fn response(self) -> axum::response::Response {
      match self.types {
          AppErrorType::Forbidden  => {
              let mut hm = HeaderMap::new();
              hm.insert(header::LOCATION, "/auth".parse().unwrap());
              (StatusCode::FOUND, hm, ()).into_response()
          }
          _ => self
              .message
              .to_owned()
              .unwrap_or("有错误发生".to_string())
              .into_response()
      }
  }
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
  }
}

impl std::error::Error for AppError {}

impl From<sea_orm::error::DbErr> for AppError {
  fn from(err: sea_orm::error::DbErr) -> Self {
      Self::from_err(Box::new(err), AppErrorType::Db)
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
      self.response()
  }
}