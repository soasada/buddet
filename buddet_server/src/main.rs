use warp::Filter;
use buddet_core::user::User;
use buddet_db::database::mongo_database::MongoDatabase;
use buddet_db::user::mongo_user_repository::MongoUserRepository;
use buddet_core::user_repository::UserRepository;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let user = User::new("John", "Smith", "test@example.org", "p4ssw0rd");
    println!("user: {}", user);
    let new_user = user.change_password("newP4ssw0rd");
    println!("user: {}", new_user);
    println!("user: {}", user);

    let mdb = MongoDatabase::new("mongodb://admin:password@localhost:27022/buddetdb", "buddetdb").await;
    let mur = MongoUserRepository::new(&mdb);
    mur.save(user);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}