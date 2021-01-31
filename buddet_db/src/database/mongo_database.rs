use mongodb::{sync::Client, error::Result, results::InsertOneResult, bson::Document};

pub struct MongoDatabase {
    client: Client,
    db_name: String,
}

impl MongoDatabase {
    pub fn new(connection_uri: &str, db_name: &str) -> std::result::Result<MongoDatabase, String> {
        match Client::with_uri_str(connection_uri) {
            Ok(mongo_client) => Ok(MongoDatabase {
                client: mongo_client,
                db_name: String::from(db_name),
            }),
            Err(err) => Err(err.to_string())
        }
    }

    pub fn upsert(&self, collection_name: &str, document: Document) -> Result<InsertOneResult> {
        self.client
            .database(self.db_name.as_str())
            .collection(collection_name)
            .insert_one(document, None)
    }
}