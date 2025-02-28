use cynic::WebSocketSubscriber;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "../forge-core/schema.graphql")]
pub struct TaskUpdatedSubscription {
    pub task_updated: Task,
}

pub async fn subscribe_to_task_updates(
    ws_url: String,
) -> Result<impl Stream<Item = Result<Task, Box<dyn std::error::Error>>>, Box<dyn std::error::Error>>
{
    let subscriber = WebSocketSubscriber::new(&ws_url).await?;

    let subscription = TaskUpdatedSubscription::build(());
    let stream = subscriber.subscribe(subscription).await?;

    Ok(stream)
}
