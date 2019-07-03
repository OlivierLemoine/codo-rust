
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Todo {
    value: String,
    is_checked: bool,
    on_change: Option<Callback<bool>>,
    on_delete: Option<Callback<()>>,
}

pub enum Msg {
    Checked,
    Del,
}

#[derive(PartialEq, Clone, Default)]
pub struct Prop {
    pub name: String,
    pub is_checked: bool,
    pub on_change: Option<Callback<bool>>,
    pub on_delete: Option<Callback<()>>,
}

impl Component for Todo {
    type Message = Msg;
    type Properties = Prop;

    fn create(p: Self::Properties, _: ComponentLink<Self>) -> Self {
        Todo {
            value: p.name,
            is_checked: p.is_checked,
            on_change: p.on_change,
            on_delete: p.on_delete,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Del => {
                if let Some(cb) = &self.on_delete {
                    cb.emit(());
                }
            }

            Msg::Checked => {
                if let Some(cb) = &self.on_change {
                    match self.is_checked {
                        true => cb.emit(false),
                        false => cb.emit(true),
                    }
                }
            }
        }
        true
    }
    fn change(&mut self, p: Self::Properties) -> ShouldRender {
        self.value = p.name;
        self.is_checked = p.is_checked;
        true
    }
}

impl Renderable<Todo> for Todo {
    fn view(&self) -> Html<Todo> {
        html! {
            <div class={format!("todo-container {}", if self.is_checked{"todo-checked"} else {""})},>
                <label>
                    <input id={format!("todo-name-{}", self.value)}, class="todo-check filled-in", type="checkbox", onchange=|_|Msg::Checked, checked={self.is_checked},/>
                    <span></span>
                </label>
                <div class="todo-txt",>{self.value.clone()}</div>
                <div class="todo-del", onclick=|_|Msg::Del,></div>
            <div/>
        }
    }
}