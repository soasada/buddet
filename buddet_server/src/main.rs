use warp::{Filter, Rejection, http::Response};
use buddet_db::database::mongo_database::MongoDatabase;
use buddet_core::user::User;
use crate::user::create_user_request::CreateUserRequest;
use buddet_db::database::{save, find};
use buddet_db::database::entity::Entity;
use buddet_db::user::UserEntity;

mod user;

#[tokio::main]
async fn main() {
    let user = User::new("John", "Smith", "test1111@example.org", "p4ssw0rd");

    if let Ok(mdb) = MongoDatabase::new("mongodb://admin:password@localhost:27022/buddetdb", "buddetdb").await {
        let result = save(&mdb, UserEntity::from(user)).await;
        match result {
            Ok(inserted) => {
                if let Some(found_user) = find(&mdb, UserEntity::collection(), inserted.as_object_id().unwrap(), UserEntity::convert_to_entity).await {
                    println!("user: {}", found_user);
                }
            }
            Err(err) => println!("{}", err.to_string())
        }

        use warp::{body, get, path, path::end, post};

        let POST = |desired_path: &str, func: fn(_) -> Result<Response, Rejection>| post().and(path(desired_path)).and(body::content_length_limit(1024 * 16)).and(body::json()).and_then(func);

        let routes = warp::any()
            // POST /register
            .or(POST("register", register));


        /*let create_user = warp::post()
            .and(warp::path("users"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|create_user_request: CreateUserRequest| {
                let user = User::new(create_user_request.firstname.as_str(), create_user_request.lastname.as_str(), create_user_request.email.as_str(), create_user_request.email.as_str());
                mur.save(&mdb, user);
                warp::reply::json(&create_user_request);
            });*/

        let hello = warp::path!("hello" / String)
            .map(|name| format!("Hello, {}!", name));

        warp::serve(hello)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
}

async fn register(request: CreateUserRequest) -> Result<Response, Rejection> {
    println!("Incoming request {}", request);
    return Ok(Response::builder().header("asd", "asd").body(""));
}