use actix_web::{web, Responder, HttpResponse};
use uuid::Uuid;
use std::sync::Mutex;
use crate::models::post::{Post, CreatePost};

pub async fn create_post(data: web::Data<Mutex<Vec<Post>>>, post: web::Json<CreatePost>) -> impl Responder {
    let mut posts = data.lock().unwrap();
    let new_post = Post::new(post.title.clone(), post.content.clone());
    posts.push(new_post.clone());
    HttpResponse::Ok().json(new_post)
}

pub async fn get_post(data: web::Data<Mutex<Vec<Post>>>, post_id: web::Path<Uuid>) -> impl Responder {
    let posts = data.lock().unwrap();
    let post = posts.iter().find(|p| p.id == *post_id);
    match post {
        Some(p) => HttpResponse::Ok().json(p),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_post(data: web::Data<Mutex<Vec<Post>>>, post_id: web::Path<Uuid>, post: web::Json<Post>) -> impl Responder {
    let mut posts = data.lock().unwrap();
    let index = posts.iter().position(|p| p.id == *post_id);
    match index {
        Some(i) => {
            posts[i].title = post.title.clone();
            posts[i].content = post.content.clone();
            posts[i].timestamp = chrono::Utc::now().timestamp();
            HttpResponse::Ok().json(posts[i].clone())
        },
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_post(data: web::Data<Mutex<Vec<Post>>>, post_id: web::Path<Uuid>) -> impl Responder {
    let mut posts = data.lock().unwrap();
    let index = posts.iter().position(|p| p.id == *post_id);
    match index {
        Some(i) => {
            posts.remove(i);
            HttpResponse::Ok().json(format!("Post {} deleted", post_id))
        },
        None => HttpResponse::NotFound().finish(),
    }
}
