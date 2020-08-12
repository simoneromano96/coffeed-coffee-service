use async_graphql::{Context, EmptySubscription, Schema, ID, FieldResult, EmptyMutation, FieldError};
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
use mongodb::Client;
use mongodb::Collection;
// use bson::doc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use crate::models::{Coffee, CoffeeModel};
use wither::{prelude::*, Result};
use wither::bson::{doc, oid::ObjectId};
// use std::time::Duration;
use futures::{Stream, StreamExt};

pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

async fn fetch_all_coffees(client: &Client) -> FieldResult<Vec<Coffee>> {
    let db = client.database("coffees");
    let mut coffees: Vec<Coffee> = Vec::new();

    let coffee_cursor = CoffeeModel::find(db.clone(), None, None).await;
    
    if let Ok(mut cursor) = coffee_cursor {
        while let Some(coffee) = cursor.next().await {
            coffees.push(coffee.unwrap().to_coffee());
        }
    }

    Ok(coffees)
}

async fn fetch_coffee_by_id(client: &Client, id: String) -> FieldResult<Coffee> {
    let db = client.database("coffees");

    let query = doc! {
        "_id": ObjectId::with_string(&id).unwrap(),
    };

    if let Some(coffee_model) = CoffeeModel::find_one(db.clone(), Some(query), None).await.unwrap() {
        Ok(coffee_model.to_coffee())
    } else {
        Err(FieldError(format!("Coffee with ID {:?} not found", id), None))
    }
}

async fn create_coffee(client: &Client, input: CoffeeInput) -> Result<Coffee> {
    let db = client.database("coffees");

    let mut coffee_model = CoffeeModel{id: None, name: input.name, price: input.price, image_url: input.image_url.into_string(), description: input.description};
    coffee_model.save(db.clone(), None).await?;

    Ok(coffee_model.to_coffee())
}

#[async_graphql::Object]
impl QueryRoot {
    async fn coffees(&self, ctx: &Context<'_>) -> FieldResult<Vec<Coffee>> {
        let client: &Client = ctx.data().unwrap();
        fetch_all_coffees(client).await
    }

    async fn coffee(&self, ctx: &Context<'_>, id: String) -> FieldResult<Coffee> {
        let client: &Client = ctx.data().unwrap();        
        fetch_coffee_by_id(client, id).await
    }
}

#[async_graphql::InputObject]
#[derive(Clone)]
pub struct CoffeeInput {
    pub name: String,
    pub price: f64,
    pub image_url: Url,
    pub description: Option<String>,
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_coffee(&self, ctx: &Context<'_>, input: CoffeeInput) -> Coffee {
        let client: &Client = ctx.data().unwrap();

        create_coffee(client, input).await.unwrap()
    }
}

/*
#[async_graphql::Enum]
enum MutationType {
    Created,
    // Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct CoffeeChanged {
    mutation_type: MutationType,
    id: ID,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[arg(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(Duration::from_secs(1)).map(move |_| {
            value += n;
            value
        })
    }

    async fn coffees(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = CoffeeChanged> {
        SimpleBroker::<CoffeeChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
*/
