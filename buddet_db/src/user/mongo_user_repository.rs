use crate::database::mongo_database::MongoDatabase;
use mongodb::{bson::Document, bson::doc};
use buddet_core::user::user::User;
use buddet_core::user::user_repository::UserRepository;
use async_trait::async_trait;
use buddet_core::error::repository_error::RepositoryErrorKind;
use buddet_core::error::repository_error::RepositoryErrorKind::SaveErr;

pub struct MongoUserRepository;

impl MongoUserRepository {
    const COLLECTION_NAME: &'static str = "user";

    pub fn new() -> MongoUserRepository {
        MongoUserRepository {}
    }

    fn convert_to_doc(user: User) -> Document {
        doc! {
          "firstname": user.firstname,
          "lastname": user.lastname,
          "email": user.email,
          "password": user.password
        }
    }
}

#[async_trait]
impl UserRepository<MongoDatabase> for MongoUserRepository {
    async fn save(&self, persistence: &MongoDatabase, user: User) -> Result<String, RepositoryErrorKind> {
        match persistence.upsert(MongoUserRepository::COLLECTION_NAME, MongoUserRepository::convert_to_doc(user)).await {
            Ok(it) => Ok(it.inserted_id.to_string()),
            Err(err) => Err(SaveErr(err.to_string()))
        }
    }

    async fn find(&self, persistence: &MongoDatabase, id: &str) -> Option<User> {
        unimplemented!()
    }
}