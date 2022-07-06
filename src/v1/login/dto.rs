/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-06 16:07:52
 * @LastEditTime: 2022-07-06 16:22:43
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\login\dto.rs
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}
