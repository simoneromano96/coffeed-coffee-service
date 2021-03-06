use async_graphql::{Context, FieldError, FieldResult, Schema, SimpleBroker, ID};
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
use mongodb::Database;
// use bson::doc;
use crate::models::{Coffee, CreateCoffeeInput, UpdateCoffeeInput};
use futures::{Stream, StreamExt};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct QueryRoot;

async fn fetch_all_coffees(db: &Database) -> FieldResult<Vec<Coffee>> {
    let mut coffees: Vec<Coffee> = Vec::new();

    let mut cursor = Coffee::find(db.clone(), None, None).await?;

    while let Some(coffee) = cursor.next().await {
        coffees.push(coffee.unwrap());
    }

    Ok(coffees)
}

async fn fetch_coffee_by_id(db: &Database, id: String) -> FieldResult<Coffee> {
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

async fn create_coffee(db: &Database, input: CreateCoffeeInput) -> FieldResult<Coffee> {
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

async fn update_coffee(db: &Database, input: UpdateCoffeeInput) -> FieldResult<Coffee> {
    use mongodb::bson::Document;
    use mongodb::options::FindOneAndUpdateOptions;

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

async fn delete_coffee(db: &Database, id: String) -> FieldResult<Coffee> {
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
    /// Returns an array with all the coffees or an empty array
    async fn coffees(&self, ctx: &Context<'_>) -> FieldResult<Vec<Coffee>> {
        let db: &Database = ctx.data()?;
        fetch_all_coffees(db).await
    }

    /// Returns a coffee by its ID, will return error if none is present with the given ID
    async fn coffee(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "ID of the coffee.")] id: String,
    ) -> FieldResult<Coffee> {
        let db: &Database = ctx.data()?;
        fetch_coffee_by_id(db, id).await
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    /// Creates a new coffee
    async fn create_coffee(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "The parameters of the new coffee.")] input: CreateCoffeeInput,
    ) -> FieldResult<Coffee> {
        let db: &Database = ctx.data()?;
        create_coffee(db, input).await
    }

    /// Updates a coffee
    async fn update_coffee(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "The parameters of the updated coffee, must have ID.")] input: UpdateCoffeeInput,
    ) -> FieldResult<Coffee> {
        let db: &Database = ctx.data()?;
        update_coffee(db, input).await
    }

    /// Deletes a coffeee
    async fn delete_coffee(&self, ctx: &Context<'_>, id: String) -> FieldResult<Coffee> {
        let db: &Database = ctx.data()?;
        delete_coffee(db, id).await
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
