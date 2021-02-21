use rust_todo_api::{create_task, establish_connection};
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)];

    create_task(&connection, name);
    println!("Task is saved");
}
