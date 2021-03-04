use std::convert::Infallible;
use buddet_core::user::User;
use buddet_db::database::{save, find};
use entity::entity::Entity;
use warp::http::StatusCode;
use mongodb::{
    Database,
    bson::{oid::ObjectId, from_document, Bson},
};

pub async fn register_handler(request: User, db: Database) -> Result<Box<dyn warp::Reply>, Infallible> {
    match save(db, request.clone()).await {
        Ok(inserted) => {
            let response = request.with_id(inserted.as_object_id().unwrap().clone());
            Ok(Box::new(warp::reply::json(&response)))
        }
        Err(err) => {
            Ok(Box::new(warp::reply::with_status(warp::reply::json(&(err.to_string())), StatusCode::BAD_REQUEST)))
        }
    }
}

pub async fn find_user(id: String, db: Database) -> Result<Box<dyn warp::Reply>, Infallible> {
    if let Ok(oid) = ObjectId::with_string(id.as_str()) {
        match find(db, "user", oid, |d| from_document::<User>(d)).await {
            Some(user) => {
                Ok(Box::new(warp::reply::json(&user.unwrap())))
            },
            None => Ok(Box::new(StatusCode::NOT_FOUND))
        }
    } else {
        Ok(Box::new(StatusCode::BAD_REQUEST))
    }
}