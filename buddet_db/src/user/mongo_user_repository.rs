use crate::database::mongo_database::MongoDatabase;
use mongodb::{bson::Document, bson::doc};
use std::borrow::Borrow;
use buddet_core::user::user::User;
use buddet_core::user::user_repository::UserRepository;

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

impl UserRepository<MongoDatabase> for MongoUserRepository {
    fn save(&self, persistence: &MongoDatabase, user: User) -> Result<String, &str> {
        match persistence.upsert(MongoUserRepository::COLLECTION_NAME, MongoUserRepository::convert_to_doc(user)) {
            Ok(it) => Ok(it.inserted_id.to_string()),
            Err(err) => {
                println!("Error: {}", err.to_string());
                return Err("Error");
            }
        }
    }

    fn find(&self, persistence: &MongoDatabase, id: &str) -> Result<User, &str> {
        unimplemented!()
    }
}