use mongodb::{Client, options::ClientOptions, error::Result, results::InsertOneResult, bson::Document};
use buddet_core::repository::Repository;
use buddet_core::repository::repository_error::RepositoryErrorKind;
use crate::database::entity::Entity;
use buddet_core::repository::repository_error::RepositoryErrorKind::SaveErr;
use async_trait::async_trait;

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

    async fn upsert(&self, collection_name: &str, document: Document) -> Result<InsertOneResult> {
        self.client
            .database(self.db_name.as_str())
            .collection(collection_name)
            .insert_one(document, None)
            .await
    }
}

#[async_trait]
impl<T: Entity> Repository<T> for MongoDatabase {
    async fn save(&self, entity: T) -> std::result::Result<String, RepositoryErrorKind> {
        match self.upsert(entity.collection(), entity.convert_to_doc()).await {
            Ok(it) => Ok(it.inserted_id.to_string()),
            Err(err) => Err(SaveErr(err.to_string()))
        }
    }

    async fn update(&self, entity: T, id: &str) -> std::result::Result<String, RepositoryErrorKind> {
        unimplemented!()
    }

    async fn delete(&self, id: &str) -> std::result::Result<String, RepositoryErrorKind> {
        unimplemented!()
    }

    async fn find(&self, id: &str) -> Option<T> {
        unimplemented!()
    }

    async fn find_all(&self) -> Vec<T> {
        unimplemented!()
    }
}