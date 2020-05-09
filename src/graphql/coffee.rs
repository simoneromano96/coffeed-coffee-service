use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema, ID};

pub type CoffeeSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct Coffee {
    id: ID,
    coffe_name: String,
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn coffees(&self, _ctx: &Context<'_>) -> Vec<Coffee> {
        let coffees = vec![Coffee {
            id: ID::from("0"),
            coffe_name: String::from("test"),
        }];

        coffees
    }
}
