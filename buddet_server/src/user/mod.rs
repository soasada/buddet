use crate::user::user_payload::{CreateUserRequest, UserCreationResponse, UserResponse};
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

mod user_payload;

pub async fn register_handler(request: CreateUserRequest, db: Database) -> Result<Box<dyn warp::Reply>, Infallible> {
    let user = User::new(
        request.firstname.as_str(),
        request.lastname.as_str(),
        request.email.as_str(),
        request.password.as_str(),
    );

    match save(db, UserEntity::from(user)).await {
        Ok(inserted) => {
            let response = UserCreationResponse {
                id: inserted.as_object_id().unwrap().to_hex()
            };
            Ok(Box::new(warp::reply::json(&response)))
        }
        Err(err) => {
            Ok(Box::new(warp::reply::with_status(warp::reply::json(&(err.to_string())), StatusCode::BAD_REQUEST)))
        }
    }
}

pub async fn find_user(id: String, db: Database) -> Result<Box<dyn warp::Reply>, Infallible> {
    if let Ok(oid) = ObjectId::with_string(id.as_str()) {
        match find(db, "user", oid, UserEntity::convert_to_entity).await {
            Some(user) => {
                Ok(Box::new(warp::reply::json(&UserResponse::new(id, user))))
            },
            None => Ok(Box::new(StatusCode::NOT_FOUND))
        }
    } else {
        Ok(Box::new(StatusCode::BAD_REQUEST))
    }
}