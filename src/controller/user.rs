use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};

use super::api_response::{ApiResponse, User, UserList};

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(ApiResponse::<String> {
        status: 404,
        message: format!("End point  not found!!!"),
        results: None,
    })
}

pub async fn add_user(payload: Json<User>, data: Data<UserList>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.insert(payload.id, payload.into_inner());

    HttpResponse::Ok().json(ApiResponse::<String> {
        status: 200,
        message: format!("User added!!!"),
        results: None,
    })
}

pub async fn fetch_users(data: Data<UserList>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_list: Vec<&User> = users.values().collect();
    HttpResponse::Ok().json(ApiResponse {
        status: 200,
        message: format!("Users fetched !!!"),
        results: Some(user_list),
    })
}
