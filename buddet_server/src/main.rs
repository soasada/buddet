use warp::Filter;
use buddet_db::database::mongo_database::MongoDatabase;
use buddet_core::user::User;
use crate::user::create_user_request::CreateUserRequest;
use buddet_db::database::{save, find};
use buddet_db::database::entity::Entity;
use buddet_db::user::UserEntity;

mod user;

#[tokio::main]
async fn main() {
    let user = User::new("John", "Smith", "test@example.org", "p4ssw0rd");

    if let Ok(mdb) = MongoDatabase::new("mongodb://admin:password@localhost:27022/buddetdb", "buddetdb").await {
        let result = save(&mdb, UserEntity::from(user)).await;
        match result {
            Ok(inserted) => {
                if let Some(foundUser) = find(&mdb, UserEntity::collection(), inserted.as_str(), UserEntity::convert_to_entity).await {
                    println!("user: {}", foundUser);
                }
            }
            Err(err) => println!("{}", err.to_string())
        }

        // POST /employees/:rate  {"name":"Sean","rate":2}
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