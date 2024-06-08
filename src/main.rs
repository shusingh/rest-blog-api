use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
use crate::models::post::Post;
use crate::routes::routes::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let posts = web::Data::new(Mutex::new(Vec::<Post>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(posts.clone())
            .configure(init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod models {
    pub mod post;
}

mod handlers {
    pub mod post_handler;
}

mod routes {
    pub mod routes;
}
