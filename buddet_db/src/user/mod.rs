/*use mongodb::{
    bson::{Document, doc, oid::ObjectId}
};
use crate::database::entity::Entity;
use buddet_core::user::User;

pub struct UserEntity {
    _id: ObjectId,
    firstname: String,
    lastname: String,
    email: String,
    password: String,
}

impl UserEntity {
    pub fn from(user: User) -> UserEntity {
        UserEntity {
            _id: ObjectId::new(),
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            password: user.password,
        }
    }
}

impl Entity for UserEntity {
    type ToEntity = User;

    fn collection() -> &'static str {
        return "user";
    }

    fn convert_to_doc(&self) -> Document {
        return doc! {
          "_id": self._id.clone(),
          "firstname": self.firstname.clone(),
          "lastname": self.lastname.clone(),
          "email": self.email.clone(),
          "password": self.password.clone()
        };
    }

    fn convert_to_entity(document: Document) -> Self::ToEntity {
        User {
            firstname: document.get("firstname").unwrap().to_string(),
            lastname: document.get("lastname").unwrap().to_string(),
            email: document.get("email").unwrap().to_string(),
            password: document.get("password").unwrap().to_string(),
        }
    }
}*/