

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// pub struct User {
//     pub id: i32,
//     pub first_name: String,
//     pub last_name: String,
//     pub email: String,
//     pub created_at: chrono::NaiveDateTime,
// }