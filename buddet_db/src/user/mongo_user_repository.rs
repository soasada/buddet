use buddet_core::user_repository::UserRepository;
use buddet_core::user::User;
use crate::database::mongo_database::MongoDatabase;
use crate::user::user_entity::UserEntity;

pub struct MongoUserRepository {
    db: &'static MongoDatabase,
}

impl MongoUserRepository {
    pub fn new(db: &'static MongoDatabase) -> MongoUserRepository {
        MongoUserRepository { db }
    }
}

impl UserRepository for MongoUserRepository {
    fn save(&self, user: User) -> Result<String, &str> {
        match self.db.upsert(UserEntity::from(user)) {
            Ok(it) => Result::Ok(it.inserted_id),
            Err(err) => Result::Err("Error")
        }
    }
}