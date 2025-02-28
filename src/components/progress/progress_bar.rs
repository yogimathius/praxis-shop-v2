use leptos::prelude::*;
use thaw::*;
use wasm_bindgen::prelude::*;

// Import CSS module
#[wasm_bindgen(
    module = "/src/components/progress/progress_bar.module.css",
    thread_local_v2
)]
extern "C" {
    static default: JsValue;
}

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
pub fn ProgressBar() -> impl IntoView {
    // Use the imported CSS module
    let _ = default;

    // Hardcoded tasks for demo
    let tasks = vec![
        Task {
            id: 1,
            title: "Complete documentation".to_string(),
            completed: true,
        },
        Task {
            id: 2,
            title: "Write unit tests".to_string(),
            completed: true,
        },
        Task {
            id: 3,
            title: "Fix UI bugs".to_string(),
            completed: false,
        },
        Task {
            id: 4,
            title: "Performance optimization".to_string(),
            completed: false,
        },
        Task {
            id: 5,
            title: "Deploy to staging".to_string(),
            completed: false,
        },
    ];

    // Hardcoded goals
    let goals = vec![
        Goal {
            id: 1,
            title: "Code Quality".to_string(),
            tasks_required: 3,
            tasks_completed: 2,
        },
        Goal {
            id: 2,
            title: "Feature Complete".to_string(),
            tasks_required: 5,
            tasks_completed: 3,
        },
        Goal {
            id: 3,
            title: "Production Ready".to_string(),
            tasks_required: 8,
            tasks_completed: 2,
        },
    ];

    view! {
        <div class="progress-container">
            <h2 class="section-title">"Goal Progress"</h2>

            <div class="goals-list">
                {goals.iter().map(|goal| {
                    let progress_percentage = (goal.tasks_completed as f32 / goal.tasks_required as f32 * 100.0).round() as i32;

                    view! {
                        <div class="goal-item">
                            <div class="goal-header">
                                <h3>{goal.title.clone()}</h3>
                                <span class="task-count">
                                    {goal.tasks_completed} "/" {goal.tasks_required} " tasks"
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

            <div class="task-list-container">
                <h3>"Tasks"</h3>
                <div class="task-list">
                    {tasks.iter().map(|task| {
                        view! {
                            <div class="task-item">
                                <thaw::Checkbox checked=task.completed/>
                                <span class="task-title" class:completed=task.completed>
                                    {task.title.clone()}
                                </span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
