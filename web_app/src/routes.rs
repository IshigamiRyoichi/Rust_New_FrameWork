use actix_web::web;
use crate::controllers::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::get().to(login::login));
    cfg.route("/index", web::post().to(index::index));
}