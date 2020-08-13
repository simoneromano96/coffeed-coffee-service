use async_graphql::{Context, FieldError, FieldResult, Schema, SimpleBroker, ID};
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
use mongodb::Client;
// use bson::doc;
use crate::models::{Coffee, CreateCoffeeInput, UpdateCoffeeInput};
use futures::{Stream, StreamExt};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct QueryRoot;

async fn fetch_all_coffees(client: &Client) -> FieldResult<Vec<Coffee>> {
    let db = client.database("coffees");
    let mut coffees: Vec<Coffee> = Vec::new();

    let mut cursor = Coffee::find(db.clone(), None, None).await?;

    while let Some(coffee) = cursor.next().await {
        coffees.push(coffee.unwrap());
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
    let mut coffee = Coffee {
        id: None,
        name: input.name,
        price: input.price,
        image_url: input.image_url.into_string(),
        description: input.description,
    };

    coffee.save(db.clone(), None).await?;

    SimpleBroker::publish(CoffeeChanged {
        mutation_type: MutationType::Created,
        id: ID::from(coffee.id.clone().unwrap().to_string()),
    });

    Ok(coffee)
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

    let opts = FindOneAndUpdateOptions::builder()
        .return_document(Some(mongodb::options::ReturnDocument::After))
        .build();

    let res: Option<Coffee> =
        Coffee::find_one_and_update(db.clone(), query, doc! {"$set": doc}, Some(opts)).await?;

    if let Some(coffee) = res {
        SimpleBroker::publish(CoffeeChanged {
            mutation_type: MutationType::Updated,
            id: ID::from(coffee.id.clone().unwrap().to_string()),
        });

        Ok(coffee)
    } else {
        Err(FieldError(
            format!("Coffee with ID {:?} not found", id),
            None,
        ))
    }
}

async fn delete_coffee(client: &Client, id: String) -> FieldResult<Coffee> {
    let db = client.database("coffees");

    let query = doc! {
        "_id": ObjectId::with_string(&id)?
    };

    let res: Option<Coffee> = Coffee::find_one_and_delete(db.clone(), query, None).await?;

    if let Some(coffee) = res {
        SimpleBroker::publish(CoffeeChanged {
            mutation_type: MutationType::Deleted,
            id: ID::from(coffee.id.clone().unwrap().to_string()),
        });

        Ok(coffee)
    } else {
        Err(FieldError(
            format!("Coffee with ID {:?} not found", id),
            None,
        ))
    }
}

#[async_graphql::Object]
impl QueryRoot {
    async fn coffees(&self, ctx: &Context<'_>) -> FieldResult<Vec<Coffee>> {
        let client: &Client = ctx.data()?;
        fetch_all_coffees(client).await
    }

    async fn coffee(&self, ctx: &Context<'_>, id: String) -> FieldResult<Coffee> {
        let client: &Client = ctx.data()?;
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
        let client: &Client = ctx.data()?;
        create_coffee(client, input).await
    }

    async fn update_coffee(
        &self,
        ctx: &Context<'_>,
        input: UpdateCoffeeInput,
    ) -> FieldResult<Coffee> {
        let client: &Client = ctx.data()?;
        update_coffee(client, input).await
    }

    async fn delete_coffee(&self, ctx: &Context<'_>, id: String) -> FieldResult<Coffee> {
        let client: &Client = ctx.data()?;
        delete_coffee(client, id).await
    }
}

#[async_graphql::Enum]
#[derive(Debug)]
enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
struct CoffeeChanged {
    mutation_type: MutationType,
    id: ID,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn coffees(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = CoffeeChanged> {
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
