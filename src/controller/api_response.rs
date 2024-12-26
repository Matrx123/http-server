use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: i32,
    pub message: String,
    pub results: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub struct UserList {
    pub users: Mutex<HashMap<u32, User>>,
}
