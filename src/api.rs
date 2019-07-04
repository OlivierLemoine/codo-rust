use crate::rocket::State;
use crate::rocket_contrib::json;
use crate::todos::{MaybeTodo, Todo};
use crate::Connection;

#[get("/todos")]
pub fn get_todos(connection: State<Connection>) -> json::JsonValue {
    let conn = connection.lock().unwrap();
    json! {Todo::get_all(&conn)}
}

#[post("/todo", data = "<todo>")]
pub fn create_todo(todo: json::Json<Todo>, connection: State<Connection>) {
    let conn = connection.lock().unwrap();
    todo.create(&conn);
}

#[put("/todo/<id>", data = "<todo>")]
pub fn update_todo(id: i32, todo: json::Json<Todo>, connection: State<Connection>) {
    let conn = connection.lock().unwrap();
    todo.update(id, &conn);
}

#[patch("/todo/<id>", data = "<maybe_todo>")]
pub fn patch_todo(id: i32, maybe_todo: json::Json<MaybeTodo>, connection: State<Connection>) {
    let conn = connection.lock().unwrap();
    Todo::patch(id, maybe_todo.clone(), &conn);
}

#[delete("/todo/<id>")]
pub fn delete_todo(id: i32, connection: State<Connection>) {
    let conn = connection.lock().unwrap();
    Todo::delete(id, &conn);
}

pub fn get_api_routes() -> Vec<rocket::Route> {
    routes![get_todos, create_todo, delete_todo, update_todo, patch_todo]
}
