use warp::Filter;
use buddet_db::database::mongo_database::MongoDatabase;
use buddet_db::user::mongo_user_repository::MongoUserRepository;
use buddet_core::user::user::User;
use buddet_core::user::user_repository::UserRepository;
use crate::user::create_user_request::CreateUserRequest;

mod user;

#[tokio::main]
async fn main() {
    let user = User::new("John", "Smith", "test2@example.org", "p4ssw0rd");

    if let Ok(mdb) = MongoDatabase::new("mongodb://admin:password@localhost:27022/buddetdb", "buddetdb").await {
        let mur = MongoUserRepository::new();
        let result = mur.save(&mdb, user).await;
        match result {
            Ok(inserted) => println!("ID: {}", inserted),
            Err(err) => println!("Error: {}", err.to_string())
        }
    }

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let create_user = warp::post()
        .and(warp::path("users"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|create_user_request: CreateUserRequest| {
            let user = User::new(create_user_request.firstname.as_str(), create_user_request.lastname.as_str(), create_user_request.email.as_str(), create_user_request.email.as_str());
            mur.save(&mdb, user);
            warp::reply::json(&create_user_request);
        });

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello.or(create_user))
        .run(([127, 0, 0, 1], 3030))
        .await;
}