/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-06 16:04:06
 * @LastEditTime: 2022-07-07 02:38:50
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\login\handle.rs
 */
use axum::{extract::Extension, Json};

use jsonwebtoken::encode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::MyResult;
use crate::share::jwt::{Claims, SECRET_KEY};
use crate::error::{AppError, AppErrorType};

use super::dto::{LoginRequest, LoginResponse};

pub async fn login_handler(
    Json(login_data): Json<LoginRequest>,
    Extension(db): Extension<DatabaseConnection>,
) -> MyResult<Json<LoginResponse>> {

    

    let auth = entity::user::Entity::find()
        .filter(entity::user::Column::Name.contains(&login_data.username))
        .filter(entity::user::Column::Password.contains(&login_data.password))
        .one(&db)
        .await;
    match auth {
        Ok(res) => match res {
            Some(res) => {
                let header = jsonwebtoken::Header::default();
                let claims = Claims::new(
                    res.uid.to_string(),
                    "axum".to_string(),
                    (60 * 60 * 24 * 7).into(),
                    "axum".to_string(),
                );
                let key = jsonwebtoken::EncodingKey::from_secret(SECRET_KEY);
                let access_token = encode(&header, &claims, &key).unwrap();
                let refresh_token = encode(&header, &claims, &key).unwrap();
                Ok(Json(LoginResponse {
                    access_token,
                    refresh_token,
                }))
            },
            None => {
                Err(AppError::from_err(
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "用户名或密码错误",
                    )),
                    AppErrorType::IncorrectLogin,
                ))
            }
        },
        Err(e) => {
            Err(AppError::from_err(
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "用户名或密码错误",
                )),
                AppErrorType::IncorrectLogin,
            ))
        }
    }
}
