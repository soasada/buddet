use mongodb::{
    bson::{Document, doc, Bson}
};
use crate::database::entity::Entity;
use buddet_core::user::User;

pub struct UserEntity;

impl Entity<User> for UserEntity {
    fn collection(&self) -> &str {
        return "user";
    }

    fn convert_to_doc(user: User) -> Document {
        doc! {
          "firstname": user.firstname,
          "lastname": user.lastname,
          "email": user.email,
          "password": user.password
        }
    }

    fn convert_to_entity(document: Document) -> User {
        User {
            firstname: document.get("firstname").unwrap().to_string(),
            lastname: document.get("lastname").unwrap().to_string(),
            email: document.get("email").unwrap().to_string(),
            password: document.get("password").unwrap().to_string(),
        }
    }
}