use actix_web::web::{self, resource, scope, ServiceConfig};

use crate::controller::user::{add_user, fetch_users, not_found};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1")
            .service(resource("/add_user").route(web::post().to(add_user)))
            .service(resource("/fetch_users").route(web::get().to(fetch_users))),
    );

    //catch all route
    cfg.default_service(web::to(not_found));
}
