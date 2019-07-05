#[derive(Debug)]
struct Todo {
    name: String,
    is_checked: bool,
}

fn main() {
    let mut todos: Vec<Todo> = vec![];
    todos.push(Todo {
        name: "a".into(),
        is_checked: false,
    });
}
