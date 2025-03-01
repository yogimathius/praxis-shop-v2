use crate::components::task::form::TaskForm;
use crate::components::task::list::TasksList;
use crate::state::use_goals::{use_goals, GoalsState};
use crate::state::use_tasks::{use_tasks, TasksState};
use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/pages/tasks/tasks.module.css")]
extern "C" {}

#[component]
pub fn TasksListPage() -> impl IntoView {
    let TasksState {
        tasks,
        loading: _,
        error: _,
        create,
        update,
        delete,
        refetch,
    } = use_tasks();
    let GoalsState {
        goals,
        loading: _,
        error: _,
        create: _,
        update: _,
        delete: _,
        refetch: _,
    } = use_goals();

    view! {
        <div class="container">
            <h2 class="tasksTitle">"Forge Operations"</h2>
            <p class="tasksSubtitle">"Shape your tasks into achievements, one strike at a time."</p>
            <TaskForm
                create=create.clone()
                refetch=refetch.clone()
                goals=goals
            />
            {
                let tasks = tasks.clone();
                move || -> View<_> {
                    let tasks = tasks.get().clone();
                    let on_toggle = update.clone();
                    let on_delete = delete.clone();
                    let on_edit = update.clone();
                    let goals = goals.clone();

                    view! {
                        <div>
                            <TasksList
                                tasks=tasks
                                on_toggle=on_toggle
                                on_delete=on_delete
                                on_edit=on_edit
                                goals=goals
                            />
                        </div>
                    }.into_view()
                }
            }
        </div>
    }
}
