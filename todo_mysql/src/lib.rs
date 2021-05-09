#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::NewTask;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_task(connection: &MysqlConnection, name: &str) {
    use schema::tasks;

    let new_task = NewTask { name };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(connection)
        .expect("Error saving new task");
}
