mod model;

use std::sync::Mutex;
use anni_repo::db::RepoDatabaseRead;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use poem::listener::TcpListener;
use poem::{get, handler, IntoResponse, Route, Server};
use async_graphql_poem::GraphQL;
use poem::web::Html;
use crate::model::AnnivQuery;

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = RepoDatabaseRead::new("./repo.db")?;

    let schema = Schema::build(AnnivQuery, EmptyMutation, EmptySubscription)
        .data(Mutex::new(manager))
        .finish();

    let app = Route::new()
        .at("/graphql", get(graphql_playground).post(GraphQL::new(schema)));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;
    Ok(())
}