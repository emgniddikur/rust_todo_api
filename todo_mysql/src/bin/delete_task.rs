extern crate diesel;
extern crate todo_mysql;

use diesel::prelude::*;
use std::env::args;
use todo_mysql::establish_connection;
use todo_mysql::schema::tasks::dsl::{name, tasks};

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(tasks.filter(name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting tasks");

    println!("Deleted {} tasks", num_deleted);
}
