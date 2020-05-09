use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema, ID};
use url::Url;

pub type CoffeeSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct Coffee {
    id: ID,
    name: String,
    price: f32,
    image_url: Url,
    description: Option<String>,
}

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

    async fn coffee(&self, _ctx: &Context<'_>, id: String) -> Coffee {
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
