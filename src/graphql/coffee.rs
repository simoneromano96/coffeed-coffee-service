use async_graphql::{Context, EmptySubscription, Schema, ID};
use nanoid::nanoid;
use serde::ser::SerializeStruct;
use mongodb::Client;
use mongodb::Collection;
use bson::doc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use futures::stream::StreamExt;
use crate::models::BaseResponse;

pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct Coffee {
    id: ID,
    name: String,
    price: f64,
    image_url: Url,
    description: Option<String>,
}

impl Coffee {
    fn to_document(&self) -> bson::Document {
        let id: String = self.id.clone().into();
        let mut document: bson::Document = doc! { "_id": id, "name": &self.name, "price": &self.price, "imageUrl": self.image_url.clone().into_string() };

        if let Some(description) = &self.description {
            document.insert("description", description);
        };

        document
    }

    pub fn from_document(coffee: bson::Document) -> Self {
        let coffee_id: &str = coffee.get_str("_id").unwrap();
        let coffee_name: &str = coffee.get_str("name").unwrap();
        let coffee_price: f64 = coffee.get_f64("price").unwrap();
        let coffee_url: Url = Url::parse(coffee.get_str("imageUrl").unwrap()).unwrap();
        let coffee_description: bson::document::ValueAccessResult<&str> = coffee.get_str("description");

        let mut result = Coffee {
            id: ID::from(coffee_id),
            name: String::from(coffee_name),
            price: coffee_price,
            image_url: coffee_url,
            description: None,
        };

        if let Ok(description) = coffee_description {
            result.description = Some(String::from(description));
        }

        result
    }
}

/*
impl Serialize for Coffee {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Coffee", 5)?;
        state.serialize_field("id", &format!("{}", &self.id))?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("price", &self.price)?;
        let image_url = self.image_url.clone();
        state.serialize_field("imageUrl", &(image_url.into_string()))?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}
*/

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn coffees(&self, ctx: &Context<'_>) -> Vec<Coffee> {
        let client: &Client = ctx.data();
        let db = client.database("coffees");
        let coffees_collection: Collection = db.collection("Coffee");
        let mut cursor: mongodb::Cursor = coffees_collection.find(None, None).await.unwrap();

        let mut coffees: Vec<Coffee> = Vec::new();

        while let Some(doc) = cursor.next().await {
            let coffee = Coffee::from_document(doc.unwrap());
            coffees.push(coffee);
        }

        coffees
    }

    async fn coffee(&self, ctx: &Context<'_>, id: String) -> BaseResponse {
        let client: &Client = ctx.data();
        let db = client.database("coffees");
        let coffees_collection: Collection = db.collection("Coffee");

        let response: BaseResponse;

        if let Some(coffee) = coffees_collection.find_one( doc! { "_id": &id }, None ).await.unwrap() {
            response = BaseResponse::with_success(Some(Coffee::from_document(coffee)));
        } else {
            response = BaseResponse::with_error(Some(String::from("No coffee found")));
        }

        response
    }
}

#[async_graphql::InputObject]
#[derive(Clone)]
pub struct CoffeeInput {
    name: String,
    price: f64,
    image_url: Url,
    description: Option<String>,
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_coffee(&self, ctx: &Context<'_>, input: CoffeeInput) -> Coffee {
        let client: &Client = ctx.data();
        let id = nanoid!();

        let db = client.database("coffees");
        let coffees_collection: Collection = db.collection("Coffee");

        let coffee = Coffee {
            id: ID::from(id.clone()),
            name: input.name,
            price: input.price,
            image_url: input.image_url,
            description: input.description,
        };

        let document: bson::Document = coffee.to_document();

        coffees_collection.insert_one(document, None).await.unwrap();

        coffee
    }
}
