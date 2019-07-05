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
