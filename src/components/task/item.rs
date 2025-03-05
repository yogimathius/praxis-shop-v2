use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::console_log;

use crate::graphql::queries::goals::Goal;
use crate::graphql::queries::tasks::Task;

#[wasm_bindgen(module = "/src/components/task/item.module.css")]
extern "C" {}

#[component]
pub fn TaskItem(
    task: Task,
    goals: ReadSignal<Vec<Goal>>,
    #[prop(into)] on_toggle: Action<Task, Result<Task, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Task, Result<Task, String>>,
) -> impl IntoView {
    let (task, _) = signal(task);

    let (status, set_status) = signal(task.get_untracked().status);
    let (is_editing, set_is_editing) = signal(false);
    let (edit_title, set_edit_title) = signal(task.get_untracked().title);
    let (edit_description, set_edit_description) =
        signal(task.get_untracked().description.unwrap_or_default());

    let title_input = NodeRef::new();
    let desc_input = NodeRef::new();

    const STATUSES: &[&str] = &["pending", "in_progress", "completed"];

    let goal_name = Memo::new(move |_| task.get().goal.as_ref().map(|g| g.title.clone()));

    let handle_save = move |_| {
        let mut updated_task = task.get();
        updated_task.title = edit_title.get();
        updated_task.description = Some(edit_description.get());
        let _ = on_edit.dispatch(updated_task);
        set_is_editing.set(false);
    };

    view! {
        <div class="task-item">
            <div class="task-wrapper">
                <div class="task-content">
                    <Show
                        when=move || is_editing.get()
                        fallback=move || {
                            view! {
                                <p class="task-title">{move || task.get().title}</p>
                                {move || task.get().description.as_ref().map(|desc| {
                                    view! {
                                        <p class="task-description">{desc.clone()}</p>
                                    }
                                })}
                            }
                        }
                    >
                        <input
                            node_ref=title_input
                            type="text"
                            class="task-edit-input"
                            value=edit_title.get()
                            on:change=move |ev| {
                                set_edit_title.set(Some(event_target_value(&ev)));
                            }
                        />
                        <input
                            node_ref=desc_input
                            type="text"
                            class="task-edit-input"
                            value=edit_description.get()
                            on:change=move |ev| {
                                set_edit_description.set(event_target_value(&ev));
                            }
                        />
                    </Show>
                </div>
            </div>
            <div class="task-right-content">
                <select
                    class="task-status-select"
                    prop:value=move || task.get().status.unwrap_or_default()
                    on:change=move |ev| {
                        let mut updated_task = task.get();
                        updated_task.status = Some(event_target_value(&ev));
                        let _ = on_toggle.dispatch(updated_task);
                    }
                >
                    <option value="pending">"Pending"</option>
                    <option value="in_progress">"In Progress"</option>
                    <option value="completed">"Completed"</option>
                </select>
                <div class="task-actions">
                    <Show
                        when=move || is_editing.get()
                        fallback=move || {
                            view! {
                                <button
                                    class="task-button task-edit-button"
                                    on:click=move |_| set_is_editing.set(true)
                                >
                                    "Edit"
                                </button>
                            }
                        }
                    >
                        <button
                            class="task-button task-save-button"
                            on:click=handle_save
                        >
                            "Save"
                        </button>
                    </Show>
                    <button
                        class="task-button task-delete-button"
                        on:click=move |_| {
                            let _ = on_delete.dispatch(task.get().id.unwrap());
                        }
                    >
                        "Delete"
                    </button>
                </div>
                {move || goal_name.get().map(|name| view! {
                    <span class="task-goal-tag">
                        {name}
                    </span>
                })}
            </div>
        </div>
    }
}
