use warp::Filter;
use buddet_core::user::User;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let a = String::from("");
    let user = User::new("Firstname", "Lastname", "test@example.org", "password");
    println!("user: {}", user);
    let newUser = user.changePassword("newPasss");
    println!("user: {}", newUser);
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}