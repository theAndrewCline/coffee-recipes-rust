use std::env;
use dotenvy::dotenv;
use diesel::{PgConnection, Connection};
use diesel::prelude::*;
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

pub mod models;
pub mod schema;

use models::Recipe;

fn create_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RecipeResult {
    recipes: Vec<Recipe>,
    count: usize,
}

#[get("/recipes")]
fn get_published_recipes() -> Json<RecipeResult> {
    use schema::recipes::dsl::*;

    let mut db = create_db_connection();

    let items = recipes
        .filter(published.eq(true))
        .limit(5)
        .load::<Recipe>(&mut db)
        .expect("Error loading posts");

    let count = items.len();

    Json(RecipeResult { recipes: items, count })
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![get_published_recipes])
}
