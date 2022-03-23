// src/handlers.rs
use super::models::{NewPost, Post};
use super::schema::posts::dsl::*;
use super::Pool;
// use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

// pub async fn get_posts() -> impl Responder {
//     format!("hello from get posts")
// }


// #[derive(Debug, Serialize, Deserialize)]
// pub struct InputPost {
//     pub title: String,
//     pub body: String,
// }

// Handler for GET /posts
pub async fn get_posts(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_all_posts(db))
        .await
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(actix_web::error::ErrorInternalServerError)?;
    )
}

fn get_all_posts(pool: web::Data<Pool>) -> Result<Vec<Post>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = posts.load::<Post>(&conn)?;
    Ok(items)
}

// Handler for POST /posts
// pub async fn add_post(
//     db: web::Data<Pool>,
//     item: web::Json<InputPost>,
// ) -> Result<HttpResponse, Error> {
//     Ok(web::block(move || add_single_post(db, item))
//         .await
//         .map(|post| HttpResponse::Created().json(post))
//         .map_err(|_| HttpResponse::InternalServerError())?)
// }

// fn add_single_post(
//     db: web::Data<Pool>,
//     item: web::Json<InputPost>,
// ) -> Result<Post, diesel::result::Error> {
//     let conn = db.get().unwrap();
//     let new_post = NewPost {
//         title: &item.title,
//         body: &item.body,
//     };
//     let res = insert_into(posts).values(&new_post).get_result(&conn)?;
//     Ok(res)
// }

// pub async fn get_post_by_id() -> impl Responder {
//     format!("hello from get posts by id")
// }

// pub async fn add_post() -> impl Responder {
//     format!("hello from add post")
// }

// pub async fn delete_post() -> impl Responder {
//     format!("hello from delete post")
// }