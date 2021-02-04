use crate::database::mongo_database::MongoDatabase;
use crate::database::entity::Entity;
use buddet_core::repository::repository_error::{RepositoryErrorKind, RepositoryErrorKind::SaveErr};
use mongodb::{
    bson::{Document, Bson}
};

pub mod entity;
pub mod mongo_database;

pub async fn save<T: Entity>(mongo_db: &MongoDatabase, entity: T) -> std::result::Result<Bson, RepositoryErrorKind> {
    match mongo_db.upsert(T::collection(), entity.convert_to_doc()).await {
        Ok(it) => Ok(it.inserted_id),
        Err(err) => Err(SaveErr(err.to_string()))
    }
}

pub async fn find<T>(mongo_db: &MongoDatabase, collection_name: &str, id: &str, conversion: fn(Document) -> T) -> Option<T> {
    match mongo_db.find(collection_name, id).await {
        Some(document) => Some(conversion(document)),
        _ => None
    }
}

/*
async fn update(&self, entity: T, id: &str) -> Result<String, RepositoryErrorKind>;
async fn delete(&self, id: &str) -> Result<String, RepositoryErrorKind>;
async fn find_all(&self) -> Vec<T>;
*/