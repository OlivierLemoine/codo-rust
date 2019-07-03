extern crate stdweb;
extern crate yew;


use stdweb::unstable::TryFrom;
use stdweb::web::{html_element::InputElement, IParentNode};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod todo;
use todo::Todo;

struct TodoHolder {
    name: String,
    is_checked: bool,
}

struct Model {
    todos: Vec<TodoHolder>,
}

enum Msg {
    None,
    AddTodo(String),
    DelTodo(usize),
    CheckTodo(usize, bool),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { todos: vec![] }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo(todo_name) => {
                self.todos.push(TodoHolder {
                    name: todo_name,
                    is_checked: false,
                });
                true
            }
            Msg::DelTodo(todo_index) => {
                self.todos.remove(todo_index);
                true
            }
            Msg::CheckTodo(todo_index, new_state) => {
                self.todos[todo_index].is_checked = new_state;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

fn todos_render(todos: &Vec<TodoHolder>) -> Html<Model> {
    html! {
        <ul>
            {
                for {
                    todos.iter().enumerate().map(|(i, todo)|html! {
                        <li>
                            <Todo: name=todo.name.clone(), is_checked=todo.is_checked, on_change=move|v|Msg::CheckTodo(i,v), on_delete=move|_|Msg::DelTodo(i),/>
                        </li>
                    })
                }
            }
        </ul>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="row",>
                <div class="col s1 m2 l3",></div>
                <div class="col s10 m8 l6",>
                    <div class="card",>
                        <div class="inp-wrapper",>
                            <input class="todo-namer",></input>
                        </div>
                        <button class="btn", style="display: inline-block", onclick=|_| {
                            let elem = stdweb::web::document().query_selector(".todo-namer").unwrap().unwrap();
                            let inp_elem = InputElement::try_from(elem).unwrap();
                            let res = inp_elem.raw_value();
                            if res == "" {Msg::None} else {
                                inp_elem.set_raw_value("");
                                Msg::AddTodo(res)
                            }
                        },>{"Add"}</button>
                        { todos_render(&self.todos) }
                        <div class="todos-stats",>
                            { format!("{} item{} left", self.todos.len(), if self.todos.len() > 1 {"s"} else {""}) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let _scope = yew::App::<Model>::new().mount_to_body();
    yew::run_loop();
}