use std::rc::Rc;

use crate::graphql::queries::tasks::Task;
use crate::services::service_context::ServiceContext;
use futures::channel::oneshot;
use leptos::prelude::*;
use leptos::task::spawn_local;

type Refetch = Box<dyn Fn() + 'static>;

pub struct TasksState {
    pub tasks: ReadSignal<Vec<Task>>,
    pub loading: ReadSignal<bool>,
    pub error: ReadSignal<Option<String>>,
    pub create: Action<Task, Result<Task, String>>,
    pub update: Action<Task, Result<Task, String>>,
    pub delete: Action<cynic::Id, Result<(), String>>,
    pub refetch: Rc<dyn Fn()>,
}

pub fn use_tasks() -> TasksState {
    let service = use_context::<ServiceContext>().expect("No service context found");
    let (tasks, set_tasks) = signal(Vec::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let service_create = service.clone();
    let service_update = service.clone();
    let service_delete = service.clone();
    let service_refetch = service.clone();

    // Initial fetch
    Effect::new(move |_| {
        let service = service.clone();
        spawn_local(async move {
            set_loading.set(true);
            match service.0.fetch_tasks().await {
                Ok(data) => set_tasks.set(data),
                Err(e) => set_error.set(Some(e)),
            }
            set_loading.set(false);
        });
    });

    // Actions
    let create = create_action(move |task: &Task| {
        let task = task.clone();
        let service = service_create.clone();

        // Return a placeholder immediately
        let (tx, rx) = futures::channel::oneshot::channel();

        spawn_local(async move {
            let result = service.0.create_task(task).await;
            let _ = tx.send(result);
        });

        async move { rx.await.unwrap_or(Err("Action canceled".to_string())) }
    });

    let update = create_action(move |task: &Task| {
        let task = task.clone();
        let service = service_update.clone();
        let id = task.id.clone().unwrap();

        let (tx, rx) = futures::channel::oneshot::channel();

        spawn_local(async move {
            let result = service.0.update_task(id, task).await;
            let _ = tx.send(result);
        });

        async move { rx.await.unwrap_or(Err("Action canceled".to_string())) }
    });

    let delete = create_action(move |id: &cynic::Id| {
        let id = id.clone();
        let service = service_delete.clone();

        let (tx, rx) = futures::channel::oneshot::channel();

        spawn_local(async move {
            let result = service.0.delete_task(id).await;
            let _ = tx.send(result);
        });

        async move { rx.await.unwrap_or(Err("Action canceled".to_string())) }
    });

    // Set up refetch function
    let refetch_fn = Rc::new(move || {
        let service = service_refetch.clone();
        set_loading.set(true);
        spawn_local(async move {
            if let Ok(data) = service.0.fetch_tasks().await {
                set_tasks.set(data);
            }
            set_loading.set(false);
        });
    });

    TasksState {
        tasks,
        loading,
        error,
        create,
        update,
        delete,
        refetch: refetch_fn,
    }
}
