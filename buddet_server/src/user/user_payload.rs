use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result};
use buddet_core::user::User;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

impl UserResponse {
    pub fn new(id: String, user: User) -> UserResponse {
        UserResponse {
            id,
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        }
    }
}