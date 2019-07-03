mod schema;
use schema::todos;
use schema::todos::dsl::*;

use diesel::{query_dsl::RunQueryDsl, Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[table_name = "todos"]
pub struct Todo {
    id: Option<i32>,
    name: String,
    is_checked: bool,
}

impl Todo {
    pub fn create(_name: String, connection: &diesel::MysqlConnection) -> Todo {
        let t = Todo {
            id: None,
            name: _name,
            is_checked: false,
        };

        diesel::insert_into(todos)
            .values(&t)
            .execute(connection)
            .unwrap();

        t
    }

    // pub fn read_all();
}