use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::components::task::item::TaskItem;
use crate::graphql::queries::goals::Goal;
use crate::graphql::queries::tasks::Task;

#[wasm_bindgen(module = "/src/components/task/list.module.css")]
extern "C" {}

#[component]
pub fn TasksList(
    tasks: Vec<Task>,
    goals: ReadSignal<Vec<Goal>>,
    #[prop(into)] on_toggle: Action<Task, Result<Task, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Task, Result<Task, String>>,
) -> impl IntoView {
    let tasks_clone = tasks.clone();

    view! {
        <div class="tasks-container">
            <h2 class="tasks-list-title">"Your Tasks"</h2>
            <Show
                when=move || !tasks_clone.is_empty()
                fallback=|| view! {
                    <div class="tasks-empty-state">
                        <h3>"No tasks yet"</h3>
                        <p>"Add your first task to get started on your journey."</p>
                    </div>
                }
            >
                <div class="tasks-list">
                    {tasks
                        .iter()
                        .map(|task| {
                            view! {
                                <TaskItem
                                    task=task.clone()
                                    on_toggle=on_toggle
                                    on_delete=on_delete
                                    on_edit=on_edit
                                    goals=goals
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </Show>
        </div>
    }
}
