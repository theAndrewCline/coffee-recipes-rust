use diesel::prelude::*;

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
