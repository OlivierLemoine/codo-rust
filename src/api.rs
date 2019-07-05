use crate::moc_bdd::Todos;
use crate::rocket::State;
use crate::rocket_contrib::json;
use crate::todo::{MaybeTodo, Todo};

#[get("/todos")]
pub fn get_todos(todos: State<Todos>) -> json::JsonValue {
    json! { todos.get_all() }
}

#[post("/todo", data = "<todo>")]
pub fn create_todo(todo: json::Json<Todo>, todos: State<Todos>) {
    todos.insert_into(todo.into_inner());
}

#[put("/todo/<id>", data = "<todo>")]
pub fn update_todo(id: i32, todo: json::Json<Todo>, todos: State<Todos>) {
    todos.update(id, todo.into_inner());
}

#[patch("/todo/<id>", data = "<maybe_todo>")]
pub fn patch_todo(id: i32, maybe_todo: json::Json<MaybeTodo>, todos: State<Todos>) {
    if let Some(mut todo) = todos.get(id) {
        todo.merge(maybe_todo.into_inner());
        todos.update(id, todo);
    }
}

#[delete("/todo/<id>")]
pub fn delete_todo(id: i32, todos: State<Todos>) {
    todos.delete(id);
}

pub fn get_api_routes() -> Vec<rocket::Route> {
    routes![get_todos, create_todo, delete_todo, update_todo, patch_todo]
}
