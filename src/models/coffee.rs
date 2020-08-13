use async_graphql::{Context, EmptySubscription, Schema, ID};
// use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
// use wither::mongodb::Client;
use wither::{prelude::*, Result};
use url::Url;

// Define a model. Simple as deriving a few traits.
#[derive(Clone, Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"name": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct Coffee {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The coffee's name.
    pub name: String,
    pub price: f64,
    pub image_url: String,
    pub description: Option<String>,
}

#[async_graphql::Object]
impl Coffee {
    async fn id(&self) -> String {
        if let Some(id) = &self.id {
            id.clone().to_string()
        } else {
            String::from("")
        }
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn price(&self) -> &f64 {
        &self.price
    }

    async fn image_url(&self) -> &str {
        &self.image_url
    }

    async fn description(&self) -> String {
        if let Some(description) = &self.description {
            description.clone()
        } else {
            String::from("")
        }
    }

    // pub async fn to_coffee(&self) -> Coffee {
    //     Coffee {
    //         id: ID::from(self.id.clone().unwrap()),
    //         name: self.name.clone(),
    //         price: self.price,
    //         image_url: self.image_url.clone(),
    //         description: self.description.clone(),
    //     }
    // }

    //pub async fn from_coffee(coffee: Coffee) -> Self {
    //    let id: String = coffee.id.into();
    //    CoffeeModel {
    //        id: Some(ObjectId::with_string(&id).unwrap()),
    //        name: coffee.name,
    //        price: coffee.price,
    //        image_url: coffee.image_url,
    //        description: coffee.description,
    //    }
    //}
}

#[async_graphql::InputObject]
#[derive(Clone)]
pub struct CreateCoffeeInput {
    pub name: String,
    pub price: f64,
    pub image_url: Url,
    pub description: Option<String>,
}

#[async_graphql::InputObject]
#[derive(Clone)]
pub struct UpdateCoffeeInput {
    pub id: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub image_url: Option<Url>,
    pub description: Option<String>,
}
