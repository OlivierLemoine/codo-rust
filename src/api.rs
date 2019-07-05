use crate::rocket::State;
use crate::rocket_contrib::json;
// use crate::todos::{MaybeTodo, Todo};

#[get("/todos")]
pub fn get_todos() -> json::JsonValue {
    // json! {Todo::get_all(&conn)}
    json! {
        vec![]
    }
}

#[post("/todo", data = "<todo>")]
pub fn create_todo(todo: json::Json<Todo>) {
}

#[put("/todo/<id>", data = "<todo>")]
pub fn update_todo(id: i32, todo: json::Json<Todo>) {
    let conn = connection.lock().unwrap();
    todo.update(id, &conn);
}

#[patch("/todo/<id>", data = "<maybe_todo>")]
pub fn patch_todo(id: i32, maybe_todo: json::Json<MaybeTodo>) {
    let conn = connection.lock().unwrap();
    Todo::patch(id, maybe_todo.clone(), &conn);
}

#[delete("/todo/<id>")]
pub fn delete_todo(id: i32) {
    let conn = connection.lock().unwrap();
    Todo::delete(id, &conn);
}

pub fn get_api_routes() -> Vec<rocket::Route> {
    routes![get_todos, create_todo, delete_todo, update_todo, patch_todo]
}
