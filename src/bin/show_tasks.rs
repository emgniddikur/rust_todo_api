extern crate diesel;
extern crate rust_todo_api;

use self::models::*;
use self::schema::tasks::dsl::*;
use diesel::prelude::*;
use rust_todo_api::*;

fn main() {
    let connection = establish_connection();
    let results = tasks
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    for task in results {
        println!("{}", task.name);
    }
}
