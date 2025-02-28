use crate::graphql::queries::goals::Goal;
use crate::graphql::schema::schema;

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "forge")]
pub struct Task {
    pub id: Option<cynic::Id>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub status: Option<String>,
    #[cynic(recurse = "1")] // Only go one level deep for goal
    pub goal: Option<Goal>,
}
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "forge", graphql_type = "RootQueryType")]
pub struct TasksQuery {
    pub tasks: Option<Vec<Option<Task>>>,
}
