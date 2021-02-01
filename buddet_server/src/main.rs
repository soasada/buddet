use warp::Filter;
use buddet_db::database::mongo_database::MongoDatabase;
use buddet_db::user::mongo_user_repository::MongoUserRepository;
use buddet_core::user::user::User;
use buddet_core::user::user_repository::UserRepository;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let user = User::new("John", "Smith", "test2@example.org", "p4ssw0rd");
    println!("user: {}", user);
    let new_user = user.change_password("newP4ssw0rd");
    println!("user: {}", new_user);
    println!("user: {}", user);

    if let Ok(mdb) = MongoDatabase::new("mongodb://admin:password@localhost:27022/buddetdb", "buddetdb").await {
        let mur = MongoUserRepository::new();
        let result = mur.save(&mdb, user).await;
        match result {
            Ok(inserted) => println!("ID: {}", inserted),
            Err(err) => println!("Error: {}", err.to_string())
        }
    }

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}