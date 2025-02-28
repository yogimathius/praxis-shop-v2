use super::tasks::Task;
use crate::graphql::schema::schema;

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "forge", graphql_type = "RootQueryType")]
pub struct GoalsQuery {
    pub goals: Option<Vec<Option<Goal>>>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "forge")]
pub struct Goal {
    pub id: Option<cynic::Id>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[cynic(rename = "tasksRequired")]
    pub tasks_required: Option<i32>,
    #[cynic(rename = "tasksCompleted")]
    pub tasks_completed: Option<i32>,
    #[cynic(recurse = "1")] // Only go one level deep for tasks
    pub tasks: Option<Vec<Option<Task>>>,
}
