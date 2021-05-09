extern crate diesel;
extern crate todo_mysql;

use self::models::*;
use self::schema::tasks::dsl::*;
use diesel::prelude::*;
use todo_mysql::*;

fn main() {
    let connection = establish_connection();
    let results = tasks
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    for task in results {
        println!("{}", task.name);
    }
}
