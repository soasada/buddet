use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: String
}