extern crate diesel;
extern crate todo_mysql;

use diesel::prelude::*;
use std::env::args;
use todo_mysql::establish_connection;
use todo_mysql::schema::tasks::dsl::{done, tasks};

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
