use async_graphql::{Context, EmptySubscription, Schema, ID};
use nanoid::nanoid;
use serde::ser::SerializeStruct;
use mongodb::Client;
use mongodb::Collection;
use bson::doc;
use serde::{Deserialize, Serialize, Serializer};
use url::Url;
pub type CoffeeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct Coffee {
    id: ID,
    name: String,
    price: f32,
    image_url: Url,
    description: Option<String>,
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
    async fn coffees(&self, _ctx: &Context<'_>) -> Vec<Coffee> {
        vec![Coffee {
            id: ID::from("0"),
            name: String::from("test"),
            price: 0.5,
            image_url: Url::parse("https://media.salon.com/2015/09/shutterstock_314135024.jpg")
                .unwrap(),
            description: None,
        }]
    }

    async fn coffee(&self, ctx: &Context<'_>, id: String) -> Coffee {
        Coffee {
            id: ID::from(id),
            name: String::from("test"),
            price: 0.5,
            image_url: Url::parse("https://media.salon.com/2015/09/shutterstock_314135024.jpg")
                .unwrap(),
            description: None,
        }
    }
}

#[async_graphql::InputObject]
#[derive(Clone)]
pub struct CoffeeInput {
    name: String,
    price: f32,
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

        let document = doc! { "_id": &id, "name": &coffee.name, "price": &coffee.price, "imageUrl": coffee.image_url.clone().into_string() };

        coffees_collection.insert_one(document, None).await.unwrap();

        coffee
    }
}
