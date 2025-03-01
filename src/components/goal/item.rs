use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::graphql::queries::goals::Goal;

#[wasm_bindgen(module = "/src/components/goal/item.module.css")]
extern "C" {}

#[component]
pub fn GoalItem(
    goal: Goal,
    #[prop(into)] on_toggle: Action<Goal, Result<Goal, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Goal, Result<Goal, String>>,
) -> impl IntoView {
    let (goal, _) = signal(goal);
    let (is_editing, set_is_editing) = signal(false);
    let (edit_title, set_edit_title) = signal(goal.get_untracked().title);
    let (edit_description, set_edit_description) =
        signal(goal.get_untracked().description.unwrap_or_default());

    let title_input = NodeRef::new();
    let desc_input = NodeRef::new();

    let handle_save = move |_| {
        let mut updated_goal = goal.get();
        updated_goal.title = edit_title.get();
        updated_goal.description = Some(edit_description.get());
        let _ = on_edit.dispatch(updated_goal);
        set_is_editing.set(false);
    };

    view! {
        <div class="goalItem">
            <div class="wrapper">
                <div class="goalContent">
                    <Show
                        when=move || is_editing.get()
                        fallback=move || {
                            view! {
                                <h3 class="goalTitle">{move || goal.get().title}</h3>
                                {move || goal.get().description.as_ref().map(|desc| {
                                    view! {
                                        <p class="description">{desc.clone()}</p>
                                    }
                                })}
                                <div class="progressInfo">
                                    <span class="progressText">
                                        "Tasks completed: "
                                        {move || goal.get().tasks_completed.unwrap_or_default()}
                                        " / "
                                        {move || goal.get().tasks_required.unwrap_or_default()}
                                    </span>
                                    <div class="progressBar">
                                        <div
                                            class="progressFill"
                                            style:width=move || {
                                                let progress = if goal.get().tasks_required.unwrap_or_default() > 0 {
                                                    (goal.get().tasks_completed.unwrap_or_default() as f32 /
                                                     goal.get().tasks_required.unwrap_or_default() as f32 * 100.0) as i32
                                                } else {
                                                    0
                                                };
                                                format!("{}%", progress)
                                            }
                                        ></div>
                                    </div>
                                </div>
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
                        let _ = on_delete.dispatch(goal.get().id.unwrap());
                    }
                >
                    "Delete"
                </button>
            </div>
        </div>
    }
}
