use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{App, HttpServer};

use self::controller::api_response::UserList;

mod controller;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = Data::new(UserList {
        users: Mutex::new(HashMap::new()),
    });
    let server = HttpServer::new(move || App::new().app_data(pool.clone()).configure(routes::init))
        .bind("127.0.0.1:3000")?;
    println!("Server is up and running!!!");
    server.run().await
}
