use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Import CSS module
#[wasm_bindgen(module = "/src/components/progress/progress_bar.module.css")]
extern "C" {}

#[derive(Clone)]
pub struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Clone)]
pub struct Goal {
    id: usize,
    title: String,
    tasks_required: usize,
    tasks_completed: usize,
}

#[component]
pub fn ProgressBar(
    #[prop(optional)] goals: Option<Vec<crate::graphql::queries::goals::Goal>>,
) -> impl IntoView {
    // Use provided goals or empty vec if None
    let goals = goals.unwrap_or_default();

    view! {
        <div class="progress-container">
            <div class="goals-list">
                {goals.iter().map(|goal| {
                    let tasks_completed = goal.tasks_completed.unwrap_or(0);
                    let tasks_required = goal.tasks_required.unwrap_or(1);
                    let progress_percentage = if tasks_required > 0 {
                        (tasks_completed as f32 / tasks_required as f32 * 100.0).round() as i32
                    } else {
                        0
                    };

                    view! {
                        <div class="goal-item">
                            <div class="goal-header">
                                <h3>{goal.title.clone().unwrap_or_default()}</h3>
                                <span class="task-count">
                                    {tasks_completed} "/" {tasks_required} " tasks"
                                </span>
                            </div>

                            <div class="progress-bar">
                                <div
                                    class="progress-fill"
                                    style=format!("width: {}%;", progress_percentage)
                                >
                                </div>
                                <div class="progress-text">
                                    {progress_percentage}"% Complete"
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
