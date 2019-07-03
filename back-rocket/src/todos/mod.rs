use crate::diesel::{query_dsl::RunQueryDsl, QueryDsl, ExpressionMethods, Insertable, Queryable, MysqlConnection};
use crate::serde::{Deserialize, Serialize};

mod schema;
use schema::todos;
use schema::todos::dsl::*;

#[derive(Queryable, Insertable, Deserialize, Serialize, Debug)]
#[table_name = "todos"]
pub struct Todo {
    id: Option<i32>,
    name: String,
    is_checked: bool,
}

impl Todo {
    pub fn create(&self, connection: &MysqlConnection) {
        diesel::insert_into(todos)
            .values(self)
            .execute(connection)
            .unwrap();
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<Todo> {
        todos::table.order(todos::id.asc()).load::<Todo>(connection).unwrap()
    }
}
