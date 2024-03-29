mod model;

use anni_repo::db::RepoDatabaseRead;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router, Server,
};
use model::{build_schema, AppSchema};
use std::net::SocketAddr;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = RepoDatabaseRead::new("./repo.db")?;
    let schema = build_schema(manager);

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9929));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
