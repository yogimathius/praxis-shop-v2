use cynic::http::ReqwestExt;
use cynic::Operation;
use reqwest::Client;
use reqwest::Response;

pub struct GraphQLClient {
    client: Client,
    endpoint: String,
}

impl GraphQLClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
        }
    }

    pub async fn query<T: cynic::QueryFragment>(
        &self,
        operation: cynic::Operation<T, T::Variables>,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await?;

        GraphQLClient::unwrap_response(response)
    }

    pub async fn mutate<T: cynic::QueryFragment>(
        &self,
        operation: T,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&self.endpoint)
            .run_graphql(operation)
            .await?;

        GraphQLClient::unwrap_response(response)
    }

    pub fn unwrap_response<T: cynic::QueryFragment>(
        response: Response,
    ) -> Result<T, Box<dyn std::error::Error>> {
        if let Some(data) = response.data {
            Ok(data)
        } else if let Some(errors) = response.errors {
            Err(format!("GraphQL Errors: {:?}", errors).into())
        } else {
            Err("No data or errors returned".into())
        }
    }
}
