use crate::user::create_user_request::CreateUserRequest;
use std::convert::Infallible;
use buddet_core::user::User;
use buddet_db::database::{save, find};
use buddet_db::database::entity::Entity;
use buddet_db::user::UserEntity;
use warp::http::StatusCode;
use mongodb::{
    Database,
    bson::oid::ObjectId,
};
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
        }
        Err(err) => {
            Ok(Box::new(warp::reply::with_status(warp::reply::json(&(err.to_string())), StatusCode::BAD_REQUEST)))
        }
    }
}

pub async fn find_user(id: ObjectId, db: Database) -> Result<Box<dyn warp::Reply>, Infallible> {
    match find(db, "user", id, UserEntity::convert_to_entity).await {
        Some(user) => Ok(Box::new(warp::reply::json(&(user.firstname)))),
        None => Ok(Box::new(StatusCode::NOT_FOUND))
    }
}