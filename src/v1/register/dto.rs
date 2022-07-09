/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-06 16:07:52
 * @LastEditTime: 2022-07-09 01:52:44
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\src\v1\register\dto.rs
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}