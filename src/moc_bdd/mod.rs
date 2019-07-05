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

pub struct Todos(Mutex<Vec<MocTodo>>);

impl Todos {
    pub fn init() -> Todos {
        Todos(vec![].into())
    }

    pub fn insert_into(&mut self, todo: todo::Todo) -> todo::Todo {
        let mut vec = self.0.lock().unwrap();
        let todo::Todo {
            id: _,
            name,
            is_checked,
        } = todo;

        let index = vec.len();

        let new_todo = MocTodo {
            id: index,
            name: name.clone(),
            is_checked,
        };

        vec.push(new_todo);

        todo::Todo {
            id: Some(index as i32),
            name,
            is_checked,
        }
    }

    pub fn get_all(&self) -> Vec<todo::Todo> {
        let vec = self.0.lock().unwrap();
        vec.iter().map(|e| e.clone().into()).collect()
    }

    pub fn update(&mut self, todo: todo::Todo) -> todo::Todo {
        let mut vec = self.0.lock().unwrap();
        let todo::Todo {
            id,
            name,
            is_checked,
        } = todo;

        let index = id.expect("An ID is required in order to update a todo") as usize;

        vec[index] = MocTodo {
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

    pub fn delete(&mut self, i: i32) {
        let mut vec = self.0.lock().unwrap();
        vec.remove(i as usize);
    }
}
