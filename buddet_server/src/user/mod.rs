use crate::user::create_user_request::CreateUserRequest;
use std::convert::Infallible;
use buddet_core::user::User;
use buddet_db::database::save;
use buddet_db::user::UserEntity;
use warp::http::StatusCode;
use mongodb::Database;
use crate::user::user_response::UserResponse;

mod create_user_request;
mod user_response;

pub async fn register_handler(request: CreateUserRequest, db: Database) -> Result<Box<dyn warp::Reply>, Infallible> {
    let user = User::new(
        request.firstname.as_str(),
        request.lastname.as_str(),
        request.email.as_str(),
        request.password.as_str(),
    );

    match save(db, UserEntity::from(user)).await {
        Ok(inserted) => {
            let response = UserResponse {
                id: inserted.as_object_id().unwrap().to_hex()
            };
            Ok(Box::new(warp::reply::json(&response)))
        },
        Err(err) => {
            Ok(Box::new(warp::reply::with_status(warp::reply::json(&(err.to_string())), StatusCode::BAD_REQUEST)))
        }
    }
}