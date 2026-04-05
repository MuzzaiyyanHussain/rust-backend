use actix_web::web;
use crate::handlers::user_handlers;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/hey", web::get().to(user_handlers::hello));
    cfg.route("/signup", web::post().to(user_handlers::signup));
    cfg.route("/login", web::post().to(user_handlers::login));
    cfg.route("/protected", web::get().to(user_handlers::protected_route));
}