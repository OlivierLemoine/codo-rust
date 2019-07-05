use crate::todo;
use std::sync::Mutex;

#[derive(Clone)]
struct MocTodo {
    id: usize,
    name: String,
    is_checked: bool,
}

impl Into<todo::Todo> for MocTodo {
    fn into(self) -> todo::Todo {
        let MocTodo {
            id,
            name,
            is_checked,
        } = self;
        todo::Todo {
            id: Some(id as i32),
            name,
            is_checked,
        }
    }
}

struct InnerTodos {
    todos: Vec<MocTodo>,
    index: usize,
}

pub struct Todos(Mutex<InnerTodos>);

impl Todos {
    pub fn init() -> Todos {
        Todos(
            (InnerTodos {
                todos: vec![],
                index: 0,
            })
            .into(),
        )
    }

    pub fn insert_into(&self, todo: todo::Todo) -> todo::Todo {
        let mut todos = self.0.lock().unwrap();
        let todo::Todo {
            id: _,
            name,
            is_checked,
        } = todo;

        let id = todos.index;
        todos.index += 1;

        let new_todo = MocTodo {
            id,
            name: name.clone(),
            is_checked,
        };

        todos.todos.push(new_todo);

        todo::Todo {
            id: Some(id as i32),
            name,
            is_checked,
        }
    }

    pub fn get(&self, id: i32) -> Option<todo::Todo> {
        let todos = self.0.lock().unwrap();
        todos
            .todos
            .iter()
            .find(|e| e.id == id as usize)
            .map(|e| e.clone().into())
    }

    pub fn get_all(&self) -> Vec<todo::Todo> {
        let todos = self.0.lock().unwrap();
        todos.todos.iter().map(|e| e.clone().into()).collect()
    }

    pub fn update(&self, todo: todo::Todo) -> todo::Todo {
        let mut todos = self.0.lock().unwrap();
        let todo::Todo {
            id,
            name,
            is_checked,
        } = todo;

        let index = id.expect("An ID is required in order to update a todo") as usize;

        let pos = todos.todos.iter().position(|e| e.id == index).unwrap();

        todos.todos[pos] = MocTodo {
            id: index,
            name: name.clone(),
            is_checked,
        };

        todo::Todo {
            id,
            name,
            is_checked,
        }
    }

    pub fn delete(&self, i: i32) {
        let mut todos = self.0.lock().unwrap();
        let pos = todos.todos.iter().position(|e| e.id == i as usize).unwrap();
        todos.todos.remove(pos);
    }
}
