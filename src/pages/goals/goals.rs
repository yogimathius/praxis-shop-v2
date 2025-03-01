use crate::state::use_goals::GoalsState;
use leptos::prelude::*;

use crate::components::goal::form::GoalForm;
use crate::components::goal::list::GoalsList;
use crate::state::use_goals::use_goals;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/goals/goals.module.css")]
extern "C" {}

#[component]
pub fn GoalsListPage() -> impl IntoView {
    let GoalsState {
        goals,
        create,
        update,
        delete,
        refetch,
        loading: _,
        error: _,
    } = use_goals();

    view! {
        <div class="container">
            <h2 class="tasksTitle">"The Anvil"</h2>
            <p class="tasksSubtitle">"Mold your goals on the anvil of determination."</p>
            <GoalForm create=create.clone() refetch=refetch.clone() />
            {
                let goals = goals.clone();
                move || -> View<_> {
                    let goals = goals.get().clone();
                    let on_delete = delete.clone();
                    let on_edit = update.clone();

                    view! {
                        <div>
                            <GoalsList
                                goals=goals
                                on_delete=on_delete
                                on_edit=on_edit
                            />
                        </div>
                    }.into_view()
                }
            }
        </div>
    }
}
