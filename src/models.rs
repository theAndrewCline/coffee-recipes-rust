use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
