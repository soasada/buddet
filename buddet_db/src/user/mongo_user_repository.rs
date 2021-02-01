use crate::database::mongo_database::MongoDatabase;
use mongodb::{bson::Document, bson::doc};
use buddet_core::user::user::User;
use buddet_core::user::user_repository::UserRepository;
use async_trait::async_trait;

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
impl<'a> UserRepository<'a, MongoDatabase> for MongoUserRepository {
    async fn save(&'a self, persistence: &MongoDatabase, user: User) -> Result<String, &'a str> {
        match persistence.upsert(MongoUserRepository::COLLECTION_NAME, MongoUserRepository::convert_to_doc(user)).await {
            Ok(it) => Ok(it.inserted_id.to_string()),
            Err(err) => {
                println!("Error: {}", err.to_string());
                return Err("Error");
            }
        }
    }

    async fn find(&'a self, persistence: &MongoDatabase, id: &str) -> Result<User, &'a str> {
        unimplemented!()
    }
}