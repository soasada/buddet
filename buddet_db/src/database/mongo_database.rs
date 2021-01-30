use mongodb::{sync::Client, error::Result, results::InsertOneResult};
use crate::database::entity::Entity;

pub struct MongoDatabase {
    client: Client,
    db_name: String,
}

impl MongoDatabase {
    pub fn new(connection_uri: &str, db_name: &str) -> MongoDatabase {
        MongoDatabase {
            client: Client::with_uri_str(connection_uri)?,
            db_name: String::from(db_name),
        }
    }

    pub fn upsert<T: Entity>(&self, entity: T) -> Result<InsertOneResult> {
        self.client
            .database(self.db_name.as_str())
            .collection(entity.name())
            .insert_one(entity.to_doc(), None)
    }
}