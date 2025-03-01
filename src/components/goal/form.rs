use std::rc::Rc;

use leptos::{ev, prelude::*, task::spawn_local};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::graphql::queries::goals::Goal;

#[wasm_bindgen(module = "/src/components/goal/form.module.css")]
extern "C" {}

#[component]
pub fn GoalForm(
    create: Action<Goal, Result<Goal, String>>,
    refetch: Rc<dyn Fn()>,
) -> impl IntoView {
    let (goal_text, set_goal_text) = signal(String::new());
    let (goal_description, set_goal_description) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (show_success, set_show_success) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let current_text = goal_text.get_untracked();
        let current_description = goal_description.get_untracked();
        if current_text.is_empty() {
            return;
        }

        set_is_submitting.set(true);
        let goal = Goal {
            id: None,
            title: Some(current_text),
            description: Some(current_description),
            tasks_required: None,
            tasks_completed: None,
            tasks: None,
        };

        set_goal_text.set(String::new());
        set_goal_description.set(String::new());
        set_show_success.set(true);
        set_is_submitting.set(false);

        spawn_local(async move {
            let _ = create.dispatch(goal);
            set_timeout(
                move || {
                    set_show_success.set(false);
                },
                std::time::Duration::from_millis(2000),
            );
        });
    };

    view! {
        <form class="form" on:submit=on_submit>
            <div class="formGroup">
                <input
                    type="text"
                    class="input"
                    placeholder="Add a new goal..."
                    on:input=move |ev| set_goal_text.set(event_target_value(&ev))
                    prop:value=move || goal_text.get()
                />
                <input
                    type="text"
                    class="input description"
                    placeholder="Add a description..."
                    on:input=move |ev| set_goal_description.set(event_target_value(&ev))
                    prop:value=move || goal_description.get()
                />
                <button
                    type="submit"
                    class="button"
                    disabled=move || is_submitting.get()
                >
                    {move || if is_submitting.get() { "Adding..." } else { "Add Goal" }}
                </button>
            </div>

            {move || show_success.get().then(|| view! {
                <div class="successMessage">
                    "Goal added successfully!"
                </div>
            })}
        </form>
    }
}
