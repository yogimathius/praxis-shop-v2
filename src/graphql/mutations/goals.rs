use crate::graphql::queries::goals::Goal;
use crate::graphql::schema::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateGoalVariables {
    pub title: String,
    pub description: Option<String>,
    pub tasks_required: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "RootMutationType",
    variables = "CreateGoalVariables"
)]
pub struct CreateGoalMutation {
    #[arguments(title: $title, description: $description, tasksRequired: $tasks_required)]
    pub create_goal: Option<Goal>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateGoalVariables {
    pub id: cynic::Id,
    pub title: String,
    pub description: Option<String>,
    pub tasks_required: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "RootMutationType",
    variables = "UpdateGoalVariables"
)]
pub struct UpdateGoalMutation {
    #[arguments(id: $id, title: $title)]
    pub update_goal: Option<Goal>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DeleteGoalVariables {
    pub id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "RootMutationType",
    variables = "DeleteGoalVariables"
)]
pub struct DeleteGoalMutation {
    #[arguments(id: $id)]
    pub delete_goal: Option<Goal>,
}
