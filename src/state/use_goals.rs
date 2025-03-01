use std::rc::Rc;

use crate::graphql::queries::goals::Goal;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::services::service_context::ServiceContext;
pub struct GoalsState {
    pub goals: ReadSignal<Vec<Goal>>,
    pub loading: ReadSignal<bool>,
    pub error: ReadSignal<Option<String>>,
    pub create: Action<Goal, Result<Goal, String>>,
    pub update: Action<Goal, Result<Goal, String>>,
    pub delete: Action<cynic::Id, Result<(), String>>,
    pub refetch: Rc<dyn Fn()>,
}

pub fn use_goals() -> GoalsState {
    let service = use_context::<ServiceContext>().expect("No service context found");
    let (goals, set_goals) = signal(Vec::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let service_create = service.clone();
    let service_update = service.clone();
    let service_delete = service.clone();
    let service_refetch = service.clone();

    // Initial fetch
    Effect::new(move |_| {
        let service = service.clone();
        set_loading.set(true);
        spawn_local(async move {
            match service.0.fetch_goals().await {
                Ok(data) => set_goals.set(data),
                Err(e) => set_error.set(Some(e)),
            }
            set_loading.set(false);
        });
    });

    // Actions
    let create = Action::new(move |goal: &Goal| {
        let goal = goal.clone();
        let service = service_create.clone();

        // Return a placeholder immediately
        let (tx, rx) = futures::channel::oneshot::channel();

        spawn_local(async move {
            let result: std::result::Result<Goal, String> = service.0.create_goal(goal).await;
            let _ = tx.send(result);
        });

        async move { rx.await.unwrap_or(Err("Action canceled".to_string())) }
    });

    let update: Action<Goal, std::result::Result<Goal, String>> =
        Action::new(move |goal: &Goal| {
            let goal = goal.clone();
            let service = service_update.clone();
            let id = goal.id.clone().unwrap();

            let (tx, rx) = futures::channel::oneshot::channel();

            spawn_local(async move {
                let result = service.0.update_goal(id, goal).await;
                let _ = tx.send(result);
            });

            async move { rx.await.unwrap_or(Err("Action canceled".to_string())) }
        });

    let delete = Action::new(move |id: &cynic::Id| {
        let id = id.clone();
        let service = service_delete.clone();

        let (tx, rx) = futures::channel::oneshot::channel();

        spawn_local(async move {
            let result = service.0.delete_goal(id).await;
            let _ = tx.send(result);
        });

        async move { rx.await.unwrap_or(Err("Action canceled".to_string())) }
    });

    // Set up refetch function
    let refetch_fn = Rc::new(move || {
        let service = service_refetch.clone();
        set_loading.set(true);
        spawn_local(async move {
            if let Ok(data) = service.0.fetch_goals().await {
                set_goals.set(data);
            }
            set_loading.set(false);
        });
    });

    GoalsState {
        goals,
        loading,
        error,
        create,
        update,
        delete,
        refetch: refetch_fn,
    }
}
