use std::sync::Arc;
use serde::{Deserialize, Serialize};
use warp::{reject, reply, Filter, Rejection, Reply};

mod auth;
mod error;

#[derive(Clone)]
pub struct User{
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_route = warp::path("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);
}

fn with_users() -> T {
    todo!()
}

fn init_users() -> T {
    todo!()
}
