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

    // Use .get_untracked() for initial values since we don't need reactivity here
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
        <div class="taskItem">
            <div class="wrapper">
                <div class="taskContent">
                    <Show
                        when=move || is_editing.get()
                        fallback=move || {
                            view! {
                                <p class="taskTitle">{move || task.get().title}</p>
                                {move || task.get().description.as_ref().map(|desc| {
                                    view! {
                                        <p class="description">{desc.clone()}</p>
                                    }
                                })}
                            }
                        }
                    >
                        <input
                            node_ref=title_input
                            type="text"
                            class="editInput"
                            value=edit_title.get()
                            on:change=move |ev| {
                                set_edit_title.set(Some(event_target_value(&ev)));
                            }
                        />
                        <input
                            node_ref=desc_input
                            type="text"
                            class="editInput"
                            value=edit_description.get()
                            on:change=move |ev| {
                                set_edit_description.set(event_target_value(&ev));
                            }
                        />
                    </Show>
                </div>
            </div>
            <div class="rightContent">
                <select
                    class="statusSelect"
                    on:change=move |ev| {
                        let new_status = event_target_value(&ev);
                        set_status.set(Some(new_status.clone()));
                        let mut updated_task = task.get();
                        updated_task.status = Some(new_status.clone());
                        updated_task.completed = Some(new_status == "completed");
                        let _ = on_toggle.dispatch(updated_task);
                    }
                    prop:value=move || status.get()
                >
                    {STATUSES.iter().map(|status| {
                        view! {
                            <option value={status.to_string()}>
                                {status.to_string().replace("_", " ")}
                            </option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
                <div class="actions">
                    <Show
                        when=move || is_editing.get()
                        fallback=move || {
                            view! {
                                <button
                                    class="button editButton"
                                    on:click=move |_| set_is_editing.set(true)
                                >
                                    "Edit"
                                </button>
                            }
                        }
                    >
                        <button
                            class="button saveButton"
                            on:click=handle_save
                        >
                            "Save"
                        </button>
                    </Show>
                    <button
                        class="button deleteButton"
                        on:click=move |_| {
                            let _ = on_delete.dispatch(task.get().id.unwrap());
                        }
                    >
                        "Delete"
                    </button>
                </div>
            </div>
            {move || goal_name.get().map(|name| view! {
                <span class="goalTag">
                    {name}
                </span>
            })}
        </div>
    }
}
