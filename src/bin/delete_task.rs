extern crate diesel;
extern crate rust_todo_api;

use diesel::prelude::*;
use rust_todo_api::establish_connection;
use rust_todo_api::schema::tasks::dsl::{name, tasks};
use std::env::args;

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(tasks.filter(name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting tasks");

    println!("Deleted {} tasks", num_deleted);
}
