use crate::diesel::{
    query_dsl::RunQueryDsl, ExpressionMethods, Insertable, MysqlConnection, QueryDsl, Queryable,
};
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
        todos::table
            .order(todos::id.asc())
            .load::<Todo>(connection)
            .unwrap()
    }

    pub fn delete(_id: i32, connection: &MysqlConnection) {
        diesel::delete(todos::table.find(_id))
            .execute(connection)
            .unwrap();
    }
}
