use std::rc::Rc;

use leptos::{ev, prelude::*, task::spawn_local};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::graphql::queries::{goals::Goal, tasks::Task};

#[wasm_bindgen(module = "/src/components/task/form.module.css")]
extern "C" {}

#[component]
pub fn TaskForm(
    create: Action<Task, Result<Task, String>>,
    refetch: Rc<dyn Fn()>,
    goals: ReadSignal<Vec<Goal>>,
) -> impl IntoView {
    let (task_text, set_task_text) = signal(String::new());
    let (task_description, set_task_description) = signal(String::new());
    let (selected_goal, set_selected_goal) = signal(None::<i32>);
    let (task_status, set_task_status) = signal("pending".to_string());
    let (is_submitting, set_is_submitting) = signal(false);
    let (show_success, set_show_success) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let current_text = task_text.get_untracked();
        let current_description = task_description.get_untracked();
        let current_goal = selected_goal.get_untracked();

        if current_text.is_empty() {
            return;
        }

        set_is_submitting.set(true);

        let task = Task {
            id: None,
            title: Some(current_text),
            description: Some(current_description),
            completed: Some(false),
            status: Some(task_status.get_untracked()),
            goal: current_goal.map(|id| Goal {
                id: Some(cynic::Id::new(id.to_string())),
                title: None,
                description: None,
                tasks_required: None,
                tasks_completed: None,
                tasks: None,
            }),
        };

        let refetch = refetch.clone();
        spawn_local(async move {
            let _ = create.dispatch(task);
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });

        set_task_text.set(String::new());
        set_task_description.set(String::new());
        set_task_status.set("pending".to_string());
        set_show_success.set(true);
        set_is_submitting.set(false);

        set_timeout(
            move || {
                set_show_success.set(false);
            },
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <form class="form" on:submit=on_submit>
            <div class="formGroup">
                <div class="inputGroup">
                    <input
                        type="text"
                        class="input"
                        placeholder="Add a new task..."
                        on:input=move |ev| set_task_text.set(event_target_value(&ev))
                        prop:value=move || task_text.get()
                    />
                    <input
                        type="text"
                        class="input description"
                        placeholder="Add a description..."
                        on:input=move |ev| set_task_description.set(event_target_value(&ev))
                        prop:value=move || task_description.get()
                    />
                </div>

                <div class="selectGroup">
                    <select
                        class="select"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_selected_goal.set(value.parse::<i32>().ok());
                        }
                    >
                        <option value="">"Select a goal (optional)"</option>
                        {move || goals.get().into_iter().map(|goal| {
                            view! {
                                <option value={goal.id.map(|id| id.inner().to_string()).unwrap_or_default()}>
                                    {goal.title.unwrap_or_default()}
                                </option>
                            }
                        }).collect_view()}
                    </select>
                    <select
                        class="select"
                        on:input=move |ev| set_task_status.set(event_target_value(&ev))
                        prop:value=move || task_status.get()
                    >
                        <option value="pending">"Pending"</option>
                        <option value="in_progress">"In Progress"</option>
                        <option value="completed">"Completed"</option>
                    </select>
                </div>
                <button
                    type="submit"
                    class="button"
                    disabled=move || is_submitting.get()
                >
                    {move || if is_submitting.get() { "Adding..." } else { "Add Task" }}
                </button>
            </div>

            {move || show_success.get().then(|| view! {
                <div class="successMessage">
                    "Task added successfully!"
                </div>
            })}
        </form>
    }
}
