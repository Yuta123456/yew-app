use yew::{function_component, html, Html, Properties};
use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todo_list: Vec<Todo>,
}
#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
  html! {
    <ul class="list-group">
        {props.todo_list.iter().map(|todo| html! {
            <TodoItem title={todo.title.clone()} completed={todo.completed} />
        }).collect::<Html>()}
    </ul>
  }
}
