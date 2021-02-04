use mongodb::{
    options::{ClientOptions, FindOneOptions},
    bson::{Document, doc},
    Client, error::Result, results::InsertOneResult,
};

pub struct MongoDatabase {
    client: Client,
    db_name: String,
}

impl MongoDatabase {
    pub async fn new(connection_uri: &str, db_name: &str) -> std::result::Result<MongoDatabase, String> {
        match ClientOptions::parse(connection_uri).await {
            Ok(client_options) => {
                match Client::with_options(client_options) {
                    Ok(mongo_client) => Ok(MongoDatabase {
                        client: mongo_client,
                        db_name: String::from(db_name),
                    }),
                    Err(err) => Err(err.to_string())
                }
            }
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn upsert(&self, collection_name: &str, document: Document) -> Result<InsertOneResult> {
        self.client
            .database(self.db_name.as_str())
            .collection(collection_name)
            .insert_one(document, None)
            .await
    }

    pub async fn find(&self, collection_name: &str, id: &str) -> Option<Document> {
        let filter = doc! { "_id": id };
        let find_options = FindOneOptions::default();
        let result = self.client
            .database(self.db_name.as_str())
            .collection(collection_name)
            .find_one(filter, find_options)
            .await;

        return match result {
            Ok(document) => document,
            Err(e) => {
                println!("{}", e.to_string());
                return Option::None;
            }
        };
    }
}