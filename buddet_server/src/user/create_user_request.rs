use serde::{Serialize, Deserialize};
use warp::Filter;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

