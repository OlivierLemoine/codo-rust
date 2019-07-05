use crate::todo;

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

pub struct Todos(Vec<MocTodo>);

impl Todos {
    pub fn init() -> Todos {
        Todos(vec![])
    }

    pub fn insert_into(&mut self, todo: todo::Todo) -> todo::Todo {
        let todo::Todo {
            id: _,
            name,
            is_checked,
        } = todo;

        let index = self.0.len();

        let new_todo = MocTodo {
            id: index,
            name: name.clone(),
            is_checked,
        };

        self.0.push(new_todo);

        todo::Todo {
            id: Some(index as i32),
            name,
            is_checked,
        }
    }

    pub fn get_all(&self) -> Vec<todo::Todo> {
        self.0.iter().map(|e| e.clone().into()).collect()
    }
}
