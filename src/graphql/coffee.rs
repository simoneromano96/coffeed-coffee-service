use async_graphql::{
    Context, EmptyMutation, EmptySubscription, FieldError, FieldResult, Schema, ID,
};
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
use mongodb::Client;
use mongodb::Collection;
// use bson::doc;
use crate::models::{Coffee, CreateCoffeeInput, UpdateCoffeeInput};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use wither::bson::{doc, oid::ObjectId};
use wither::{prelude::*, Result};
// use std::time::Duration;
use futures::{Stream, StreamExt};

pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

async fn fetch_all_coffees(client: &Client) -> FieldResult<Vec<Coffee>> {
    let db = client.database("coffees");
    let mut coffees: Vec<Coffee> = Vec::new();

    let coffee_cursor = Coffee::find(db.clone(), None, None).await;

    if let Ok(mut cursor) = coffee_cursor {
        while let Some(coffee) = cursor.next().await {
            coffees.push(coffee.unwrap());
        }
    }

    Ok(coffees)
}

async fn fetch_coffee_by_id(client: &Client, id: String) -> FieldResult<Coffee> {
    let db = client.database("coffees");

    let query = doc! {
        "_id": ObjectId::with_string(&id)?,
    };

    if let Some(coffee_model) = Coffee::find_one(db.clone(), Some(query), None).await? {
        Ok(coffee_model)
    } else {
        Err(FieldError(
            format!("Coffee with ID {:?} not found", id),
            None,
        ))
    }
}

async fn create_coffee(client: &Client, input: CreateCoffeeInput) -> FieldResult<Coffee> {
    let db = client.database("coffees");
    let mut coffee_model = Coffee {
        id: None,
        name: input.name,
        price: input.price,
        image_url: input.image_url.into_string(),
        description: input.description,
    };

    coffee_model.save(db.clone(), None).await?;

    Ok(coffee_model)
}

async fn update_coffee(client: &Client, input: UpdateCoffeeInput) -> FieldResult<Coffee> {
    use mongodb::bson::Document;
    use mongodb::options::FindOneAndUpdateOptions;

    let db = client.database("coffees");

    let mut doc: Document = Document::new();

    let id = input.id;

    let query = doc! {
        "_id": ObjectId::with_string(&id)?
    };

    if let Some(name) = input.name {
        doc.insert("name", name);
    }

    if let Some(price) = input.price {
        doc.insert("price", price);
    }

    if let Some(description) = input.description {
        doc.insert("description", description);
    }

    if let Some(image_url) = input.image_url {
        doc.insert("imageUrl", image_url.into_string());
    }

    let opts = FindOneAndUpdateOptions::builder().return_document(Some(mongodb::options::ReturnDocument::After)).build();
    Ok(Coffee::find_one_and_update(db.clone(), query, doc! {"$set": doc}, Some(opts)).await?.unwrap())
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

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_coffee(
        &self,
        ctx: &Context<'_>,
        input: CreateCoffeeInput,
    ) -> FieldResult<Coffee> {
        let client: &Client = ctx.data().unwrap();

        create_coffee(client, input).await
    }

    async fn update_coffee(&self, ctx: &Context<'_>, input: UpdateCoffeeInput) -> FieldResult<Coffee> {
        let client: &Client = ctx.data().unwrap();

        update_coffee(client, input).await
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
