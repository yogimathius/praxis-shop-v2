use leptos::{prelude::use_context, *};
use std::sync::Arc;

use crate::services::graphql_service::GraphQLService;

#[derive(Clone)]
pub struct ServiceContext(pub Arc<GraphQLService>);

pub fn use_service() -> ServiceContext {
    use_context::<ServiceContext>().expect("ServiceContext not provided")
}
