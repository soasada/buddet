use entity::entity::Entity;
use buddet_core::repository::repository_error::{RepositoryErrorKind, RepositoryErrorKind::SaveErr};
use mongodb::{
    bson::{Document, Bson, doc, oid::ObjectId, to_document},
    Database,
    options::FindOneOptions,
};
use serde::{Serialize};

pub async fn save<T: Serialize>(db: Database, entity: T) -> Result<Bson, RepositoryErrorKind> {
    let result = db
        .collection("user")
        .insert_one(to_document(&entity).unwrap(), None)
        .await;

    match result {
        Ok(it) => Ok(it.inserted_id),
        Err(err) => Err(SaveErr(err.to_string()))
    }
}

pub async fn find<T>(db: Database,
                     collection_name: &str,
                     id: ObjectId,
                     conversion: fn(Document) -> T) -> Option<T> {
    let filter = doc! { "_id": id };
    let find_options = FindOneOptions::default();
    let result = db
        .collection(collection_name)
        .find_one(filter, find_options)
        .await;

    match result {
        Ok(document) => Some(conversion(document.unwrap())),
        Err(e) => {
            println!("{}", e.to_string());
            return Option::None;
        }
    }
}

/*
async fn update(&self, entity: T, id: &str) -> Result<String, RepositoryErrorKind>;
async fn delete(&self, id: &str) -> Result<String, RepositoryErrorKind>;
async fn find_all(&self) -> Vec<T>;
*/