use crate::diesel::{query_dsl::RunQueryDsl, Insertable, Queryable};
use crate::serde::{Deserialize, Serialize};

mod schema;
use schema::todos;
use schema::todos::dsl::*;


#[derive(Queryable, Insertable, Deserialize, Serialize)]
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