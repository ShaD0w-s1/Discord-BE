/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-06 16:04:06
 * @LastEditTime: 2022-07-09 20:33:09
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\login\handle.rs
 */
use axum::{extract::Extension, Json};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::error::{Error, MyResult};

use super::dto::{LoginRequest, LoginResponse};

pub async fn login_handler(
    Json(login_data): Json<LoginRequest>,
    Extension(db): Extension<DatabaseConnection>,
) -> MyResult<Json<LoginResponse>> {
    let auth = entity::user::Entity::find()
        .filter(entity::user::Column::Name.eq(login_data.username.as_str()))
        .filter(entity::user::Column::Password.eq(login_data.password.as_str()))
        .one(&db)
        .await
        .map_err(|error| Error::DbError(error))?
        .ok_or(Error::UserNotFound)?;

    Ok(Json(LoginResponse {
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    }))
}
