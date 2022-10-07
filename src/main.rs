use std::env;
use dotenvy::dotenv;
use diesel::{PgConnection, Connection};
use diesel::prelude::*;

pub mod models;
pub mod schema;

use models::Recipe;

fn create_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use schema::recipes::dsl::*;

    let connection = &mut create_db_connection();
    let results = recipes
        .filter(published.eq(true))
        .limit(5)
        .load::<Recipe>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
