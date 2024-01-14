use actix_web::web;

use super::user_routes::{get_all_users, get_user, add_user, update_user, delete_user};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
            .service(get_all_users)
            .service(get_user)
            .service(add_user)
            .service(update_user)
            .service(delete_user)
            ;
    conf.service(scope);            
}