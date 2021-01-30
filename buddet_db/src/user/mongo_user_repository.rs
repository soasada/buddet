use buddet_core::user_repository::UserRepository;
use buddet_core::user::User;
use crate::database::database::Database;
use crate::database::mongo_database::MongoDatabase;
use crate::user::user_entity::UserEntity;

pub struct MongoUserRepository {
    db: &'static MongoDatabase,
}

impl MongoUserRepository {
    pub fn new(db: &MongoDatabase) -> MongoUserRepository {
        MongoUserRepository { db }
    }
}

impl UserRepository for MongoUserRepository {
    fn save(&self, user: User) -> String {
        self.db.upsert(UserEntity::from(user))
    }
}