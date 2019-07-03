use crate::rocket::State;
use crate::rocket_contrib::json;
use crate::todos::Todo;
use crate::Connection;

use std::ops::Deref;

#[get("/todos")]
pub fn get_todos(connection: State<Connection>) -> json::JsonValue {
    let conn = connection.lock().unwrap();
    json! {Todo::read_all(&conn)}
}

#[post("/todo", data = "<todo>")]
pub fn create_todo(todo: json::Json<Todo>, connection: State<Connection>) -> json::JsonValue {
    let conn = connection.lock().unwrap();
    todo.create(&conn);
    json! {todo.deref()}
}

#[delete("/todo/<id>")]
pub fn delete_todo(id: i32, connection: State<Connection>) {
    let conn = connection.lock().unwrap();
    Todo::delete(id, &conn);
}

pub fn get_api_routes() -> Vec<rocket::Route> {
    routes![get_todos, create_todo, delete_todo]
}
