/*
* @Author: zhangyuxuan
* @Date: 2022-07-06 16:04:06
 * @LastEditTime: 2022-07-09 20:53:46
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\register\handle.rs
*/
use axum::{extract::Extension, http::StatusCode, Json};



use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::error::{Error, MyResult};

use super::dto::RegisterRequest;

pub async fn register_handler(
    Json(register_data): Json<RegisterRequest>,
    Extension(db): Extension<DatabaseConnection>,
) -> MyResult<(StatusCode, Json<()>)> {
    let auth = entity::user::Entity::find()
        .filter(entity::user::Column::Name.eq(register_data.username.as_str()))
        .filter(entity::user::Column::Password.eq(register_data.password.as_str()))
        .one(&db)
        .await
        .map_err(|error| Error::DbError(error))?
        .ok_or(Error::UserNotFound)?;
    Ok((StatusCode::CREATED, Json(())))
}
