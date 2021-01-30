use crate::database::entity::Entity;
use mongodb::{error::Result, results::InsertOneResult};
use async_trait::async_trait;

#[async_trait]
pub trait Database {
    async fn upsert<T: Entity>(&self, entity: T) -> String;
}
