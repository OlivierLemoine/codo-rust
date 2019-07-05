use crate::serde::{Deserialize, Serialize};

mod schema;
use schema::todos;
use schema::todos::dsl::*;

#[derive(Deserialize, Clone)]
pub struct MaybeTodo {
    name: Option<String>,
    is_checked: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, AsChangeset)]
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

    pub fn update(&self, _id: i32, connection: &MysqlConnection) {
        diesel::update(todos::table.find(_id))
            .set(self)
            .execute(connection)
            .unwrap();
    }

    pub fn patch(_id: i32, mt: MaybeTodo, connection: &MysqlConnection) {
        let mut tmp_todos: Vec<Todo> = todos::table.find(_id).load::<Todo>(connection).unwrap();
        if let Some(todo) = tmp_todos.first_mut() {
            todo.patch_from(mt);
            todo.update(_id, connection);
        }
    }

    pub fn get_all(connection: &MysqlConnection) -> Vec<Todo> {
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

    fn patch_from(&mut self, mt: MaybeTodo) {
        if let Some(_name) = mt.name {
            self.name = _name;
        }
        if let Some(_is_checked) = mt.is_checked {
            self.is_checked = _is_checked;
        }
    }
}
