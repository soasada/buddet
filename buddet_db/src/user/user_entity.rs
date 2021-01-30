use mongodb::{bson::oid::ObjectId, bson::Document, bson::doc};
use crate::database::entity::Entity;
use buddet_core::user::User;

pub struct UserEntity {
    id: ObjectId,
    firstname: String,
    lastname: String,
    email: String,
    password: String,
}

impl UserEntity {
    pub fn from(user: User) -> UserEntity {
        UserEntity {
            id: ObjectId::new(),
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        }
    }
}

impl Entity for UserEntity {
    fn name(&self) -> &str {
        "user"
    }

    fn to_doc(&self) -> Document {
        doc! {
          "id": self.id.clone(),
          "firstname": self.firstname.clone(),
          "lastname": self.lastname.clone(),
          "email": self.email.clone(),
          "password": self.password.clone()
        }
    }
}