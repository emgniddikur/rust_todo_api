use super::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: u64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub name: &'a str,
}
