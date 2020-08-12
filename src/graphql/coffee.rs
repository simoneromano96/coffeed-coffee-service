use async_graphql::{Context, EmptySubscription, Schema, ID, EmptyMutation};
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
use mongodb::Client;
use mongodb::Collection;
// use bson::doc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use futures::stream::StreamExt;
use crate::models::{Coffee, CoffeeModel};
use wither::{prelude::*, Result};
use wither::bson::{doc, oid::ObjectId};

pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

async fn fetch_all_coffees(client: &Client) -> Result<Vec<Coffee>> {
    let db = client.database("coffees");
    let mut coffees: Vec<Coffee> = Vec::new();

    let mut cursor = CoffeeModel::find(db.clone(), None, None).await?;
    while let Some(coffee) = cursor.next().await {
        // let c: CoffeeModel = coffee;
        // println!("{:?}", user);
        coffees.push(coffee.unwrap().to_coffee());
    }

    Ok(coffees)
}

async fn fetch_coffee_by_id(client: &Client, id: String) -> Result<Coffee> {
    let db = client.database("coffees");

    let query = doc! {
        "_id": ObjectId::with_string(&id).unwrap(),
    };

    println!("{:?}", query);

    let coffee_model = CoffeeModel::find_one(db.clone(), Some(query), None).await?;
    
    println!("{:?}", coffee_model);

    Ok(coffee_model.unwrap().to_coffee())
}

async fn create_coffee(client: &Client, input: CoffeeInput) -> Result<Coffee> {
    let db = client.database("coffees");

    // Create a Coffee.
    let mut coffee_model = CoffeeModel{id: None, name: input.name, price: input.price, image_url: input.image_url.into_string(), description: input.description};
    coffee_model.save(db.clone(), None).await?;

    Ok(coffee_model.to_coffee())
}

#[async_graphql::Object]
impl QueryRoot {
    async fn coffees(&self, ctx: &Context<'_>) -> Vec<Coffee> {
        let client: &Client = ctx.data().unwrap();
        // let db = client.database("coffees");
        // let coffees_collection: Collection = db.collection("Coffee");
        // let mut cursor: mongodb::Cursor = coffees_collection.find(None, None).await.unwrap();
        fetch_all_coffees(client).await.unwrap()

        // coffees
    }

    async fn coffee(&self, ctx: &Context<'_>, id: String) -> Coffee {
        let client: &Client = ctx.data().unwrap();

        fetch_coffee_by_id(client, id).await.unwrap()
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
