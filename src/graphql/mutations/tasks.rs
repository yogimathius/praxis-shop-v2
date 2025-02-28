use crate::graphql::queries::tasks::Task;
use crate::graphql::schema::schema;
use cynic::{MutationBuilder, QueryFragment, QueryVariables};

#[derive(QueryVariables, Debug)]
pub struct CreateTaskVariables {
    pub title: String,
    pub description: Option<String>,
    pub goal_id: Option<cynic::Id>,
    // pub status: Option<String>,
    // pub completed: Option<bool>,
}

#[derive(QueryFragment, Debug)]
#[cynic(
    schema = "forge",
    graphql_type = "RootMutationType",
    variables = "CreateTaskVariables"
)]
pub struct CreateTaskMutation {
    #[arguments(
        title: $title,
        description: $description,
        goalId: $goal_id
    )]
    pub create_task: Option<Task>,
}

// Helper function to build the mutation
pub fn build_mutation(
    title: String,
    description: Option<String>,
    goal_id: Option<cynic::Id>,
) -> cynic::Operation<CreateTaskMutation, CreateTaskVariables> {
    CreateTaskMutation::build(CreateTaskVariables {
        title,
        description,
        goal_id,
    })
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateTaskVariables {
    pub id: cynic::Id,
    pub title: String,
    pub description: Option<String>,
    pub goal_id: Option<cynic::Id>,
    // pub status: Option<String>,
    // pub completed: Option<bool>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "RootMutationType",
    variables = "UpdateTaskVariables"
)]
pub struct UpdateTaskMutation {
    #[arguments(id: $id, title: $title, description: $description)]
    pub update_task: Option<Task>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DeleteTaskVariables {
    pub id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "RootMutationType",
    variables = "DeleteTaskVariables"
)]
pub struct DeleteTaskMutation {
    #[arguments(id: $id)]
    pub delete_task: Option<Task>,
}
