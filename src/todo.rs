use crate::serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct MaybeTodo {
    name: Option<String>,
    is_checked: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub id: Option<i32>,
    pub name: String,
    pub is_checked: bool,
}

impl Todo {
    pub fn merge(&mut self, t: MaybeTodo) {
        let MaybeTodo { name, is_checked } = t;
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(is_checked) = is_checked {
            self.is_checked = is_checked;
        }
    }
}
