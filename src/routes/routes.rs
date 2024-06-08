use actix_web::web;
use crate::handlers::post_handler::{create_post, get_post, update_post, delete_post};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}", web::put().to(update_post))
            .route("/{id}", web::delete().to(delete_post))
    );
}
