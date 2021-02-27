extern crate diesel;
extern crate rust_todo_api;

use diesel::prelude::*;
use rust_todo_api::establish_connection;
use rust_todo_api::schema::tasks::dsl::{done, tasks};
use std::env::args;

fn main() {
    let id = args()
        .nth(1)
        .expect("complete_task requires a task id")
        .parse::<u64>()
        .expect("Invalid ID");
    let connection = establish_connection();

    diesel::update(tasks.find(id))
        .set(done.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find task {}", id));
}
