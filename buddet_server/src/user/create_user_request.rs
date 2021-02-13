use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

impl Display for CreateUserRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ \
        \"firstname\": {}, \
        \"lastname\": {}, \
        \"email\": {}, \
        \"password\": {} \
        }}", self.firstname, self.lastname, self.email, self.password)
    }
}