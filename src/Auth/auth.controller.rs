/*
 * @Author: zhangyuxuan
 * @Date: 2022-06-13 23:04:58
 * @LastEditTime: 2022-06-15 05:41:41
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-FEd:\Git\Discord-BE\src\Auth\auth.controller.rs
 */
use std::string;

use rocket::routes;

struct loginData {
  name: String,
  password: String
}


struct registerData {
  name: String,
  password: String
}

fn routes() -> routes{
  return routes![login,register]
  
}

#[macro_use]
extern crate rocket;


#[post("/auth/login", data = data)]
fn login(data: loginData) -> &'static str {
    "Hello, world!"
}

#[post("/auth/register", data = data)]
fn register(data:registerData) -> &'static str {
    "Hello, world!"
}