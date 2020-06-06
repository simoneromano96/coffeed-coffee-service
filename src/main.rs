#![feature(associated_type_bounds)]

mod graphql;
mod models;

use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot};

use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GQLResponse},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::GQLRequest;
// use std::sync::Arc;

async fn index(schema: web::Data<CoffeeSchema>, gql_request: GQLRequest) -> web::Json<GQLResponse> {
    web::Json(GQLResponse(gql_request.into_inner().execute(&schema).await))
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Connect to the database.
    // let client = Arc::new(lucid_client::LucidKVClient::new(None));
    use mongodb::Client;
    let client: Client = Client::with_uri_str("mongodb://root:example@localhost:27017/admin").await.unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(client)
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
