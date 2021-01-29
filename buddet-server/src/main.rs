use warp::Filter;
use buddet_core::user::User;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let username = String::from("Nico");
    let user = User { name: username };
    println!("user: {}", user.name);
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}