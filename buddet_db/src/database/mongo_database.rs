use mongodb::{Client, options::ClientOptions, error::Result, results::InsertOneResult};
use crate::database::database::Database;
use crate::database::entity::Entity;
use std::future::Future;

pub struct MongoDatabase {
    client: Client,
    db_name: String,
}

impl MongoDatabase {
    pub async fn new(connection_uri: &str, db_name: &str) -> Result<MongoDatabase> {
        let mut client_options = ClientOptions::parse(connection_uri).await?;

        Result::Ok(MongoDatabase {
            client: Client::with_options(client_options)?,
            db_name: String::from(db_name),
        })
    }
}

impl Database for MongoDatabase {
    async fn upsert<T: Entity>(&self, entity: T) -> String {
        self.client
            .database(self.db_name.as_str())
            .collection(entity.name())
            .insert_one(entity.to_doc(), None)
            .await?
            .inserted_id
    }
}