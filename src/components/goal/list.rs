use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::components::goal::item::GoalItem;
use crate::graphql::queries::goals::Goal;
#[wasm_bindgen(module = "/src/components/goal/list.module.css")]
extern "C" {}

#[component]
pub fn GoalsList(
    goals: Vec<Goal>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Goal, Result<Goal, String>>,
) -> impl IntoView {
    view! {
        <div class="goalsContainer">
            <h2 class="listTitle">"Your Goals"</h2>
            <div class="goalsList">
                {goals
                    .iter()
                    .map(|goal| {
                        view! {
                            <GoalItem
                                goal=goal.clone()
                                on_delete=on_delete
                                on_edit=on_edit
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
