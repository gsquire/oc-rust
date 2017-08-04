#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate rocket;
extern crate serde_json;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use schema::reviews;
use rocket_contrib::{Json, Value};

mod schema;

#[derive(Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "reviews"]
struct Review {
    id: Option<i32>,
    rating: i32,
    name: String,
    body: String,
}

fn make_conn() -> SqliteConnection {
    dotenv().ok();

    let db = env::var("DATABASE_URL").expect("database URL must be set");
    SqliteConnection::establish(&db).expect(&format!("error connecting to {}", db))
}

#[get("/<rate>")]
fn fetch(rate: i32) -> Json<Value> {
    use schema::reviews::dsl::*;

    let conn = make_conn();

    let filtered_reviews = reviews
        .filter(rating.ge(rate))
        .load::<Review>(&conn)
        .expect("error loading reviews");
    Json(json!(filtered_reviews))
}

#[post("/new", format = "application/json", data = "<review>")]
fn new(review: Json<Review>) -> Json<Value> {
    let conn = make_conn();
    let _ = diesel::insert(&review.0).into(reviews::table).execute(
        &conn,
    );
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite().mount("/", routes![fetch, new]).launch();
}
