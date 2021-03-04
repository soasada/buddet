use serde::{Serialize, Deserialize};
use entity_derive::Entity;
use entity::entity::Entity;
use mongodb::{bson::{Document, doc, oid::ObjectId}};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Entity, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(firstname: &str, lastname: &str, email: &str, password: &str) -> User {
        User {
            _id: ObjectId::new(),
            firstname: String::from(firstname),
            lastname: String::from(lastname),
            email: String::from(email),
            password: String::from(password),
        }
    }

    pub fn with_id(&self, _id: ObjectId) -> User {
        User {
            _id: _id,
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ \
        \"firstname\": {}, \
        \"lastname\": {}, \
        \"email\": {}, \
        \"password\": {} \
        }}", self.firstname, self.lastname, self.email, self.password)
    }
}
