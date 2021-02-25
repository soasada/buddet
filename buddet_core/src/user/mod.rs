use serde::{Serialize, Deserialize};
use entity_derive::entity_derive;
use entity::entity::Entity;
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Entity)]
pub struct User {
    #[serde(default = "default_id")]
    pub _id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

fn default_id() -> String {
    "".to_string()
}

impl User {
    pub fn new(firstname: &str, lastname: &str, email: &str, password: &str) -> User {
        User {
            _id: String::from(""),
            firstname: String::from(firstname),
            lastname: String::from(lastname),
            email: String::from(email),
            password: String::from(password),
        }
    }

    pub fn with_id(&self, _id: &str) -> User {
        User {
            _id: String::from(_id),
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
