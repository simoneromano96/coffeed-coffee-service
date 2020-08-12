use async_graphql::{Context, EmptySubscription, Schema, ID};
// use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
// use wither::mongodb::Client;
use wither::{prelude::*, Result};
use url::Url;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct Coffee {
    pub id: ID,
    pub name: String,
    pub price: f64,
    pub image_url: String,
    pub description: Option<String>,
}

// Define a model. Simple as deriving a few traits.
#[derive(Clone, Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"name": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct CoffeeModel {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The coffee's name.
    pub name: String,
    pub price: f64,
    pub image_url: String,
    pub description: Option<String>,
}

impl CoffeeModel {
    pub fn to_coffee(&self) -> Coffee {
        Coffee {
            id: ID::from(self.id.clone().unwrap()),
            name: self.name.clone(),
            price: self.price,
            image_url: self.image_url.clone(),
            description: self.description.clone(),
        }
    }

    pub fn from_coffee(coffee: Coffee) -> Self {
        let id: String = coffee.id.into();

        CoffeeModel {
            id: Some(ObjectId::with_string(&id).unwrap()),
            name: coffee.name,
            price: coffee.price,
            image_url: coffee.image_url,
            description: coffee.description,
        }
    }
}