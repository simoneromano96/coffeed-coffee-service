#![feature(associated_type_bounds)]

mod graphql;
mod models;

use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use mongodb::bson::doc;
// use std::sync::Arc;

async fn index(schema: web::Data<CoffeeSchema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn index_ws(
    schema: web::Data<CoffeeSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}

async fn init() -> wither::mongodb::Database {
    // use models::Coffee;
    use wither::mongodb::Client;
    // Connect to the database.
    // let client = Arc::new(lucid_client::LucidKVClient::new(None));
    let client: Client = Client::with_uri_str("mongodb://root:example@localhost:27017/admin")
        .await
        .unwrap();

    // Coffee::sync(client.clone()).await.unwrap();

    let db = client.database("coffees");
    db.run_command(
        doc! {
            "createIndexes": "coffees",
            "indexes": [
                {
                "key": {
                    "name": 1,
                },
                "name": "nameIndex",
                "unique": true,
                },
            ],
        },
        None,
    )
    .await.unwrap();

    db
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = init().await;

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(db)
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
