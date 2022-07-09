/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-07 02:17:28
 * @LastEditTime: 2022-07-09 18:52:37
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\share\middleware.rs
 */
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use crate::error::{AppError, Error};

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::share::jwt::{Claims, SECRET_KEY};

pub struct Auth(pub String);
#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = AppError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|err| Error::AxumTypedHeaderError(err))?;
        decode::<Claims>(
            &bearer.token().to_string(),
            &DecodingKey::from_secret(SECRET_KEY),
            &Validation::default(),
        )
        .map_err(|err| Error::JwtError(err))?;
        Ok(Auth(bearer.token().to_string()))
    }
}
