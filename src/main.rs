use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;
use components::todo::todo_form::TodoForm;
use crate::components::todo::types::Todo;
mod components;
#[function_component(App)]
fn app() -> Html {
    let todo_list = use_state(|| create_todo_list());
    let next_id = use_state(|| 1);

    let on_add = {
        let todo_list = todo_list.clone();
        Callback::from(move |title: String| {
            let mut current_todo_list = (*todo_list).clone();
            current_todo_list.push(Todo {
                id: *next_id,
                title,
                completed: false,
            });
            next_id.set(*next_id + 1);
            todo_list.set(current_todo_list);
        })
    };
    html! {
        <>
            <Header/>
            <main class="container-fluid mt-2">
                <TodoForm {on_add}/>
                <TodoList todo_list={(*todo_list).clone()}/>
            </main>
        </>
    }
}
fn create_todo_list() -> Vec<Todo> {
    let mut todo_list: Vec<Todo> = vec![];
    todo_list.push(Todo {
        id: 1,
        title: "Learn Rust".to_string(),
        completed: false,
    });

    todo_list.push(Todo {
        id: 2,
        title: "Learn Ruby".to_string(),
        completed: false,
    });

    todo_list.push(Todo {
        id: 3,
        title: "Learn Rails".to_string(),
        completed: false,
    });

    return todo_list;
}
fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}