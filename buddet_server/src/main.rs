use mongodb::{options::ClientOptions, Client};

mod user;

#[tokio::main]
async fn main() {
    match ClientOptions::parse("mongodb://admin:password@localhost:27022").await {
        Ok(client_options) => {
            match Client::with_options(client_options) {
                Ok(mongo_client) => {
                    let db = mongo_client.database("buddetdb");
                    let api = filters::new(db);

                    warp::serve(api)
                        .run(([127, 0, 0, 1], 3030))
                        .await;
                }
                Err(err) => println!("{}", err.to_string())
            }
        }
        Err(err) => println!("{}", err.to_string())
    }
}

mod filters {
    use super::user::register_handler;
    use warp::{Filter, body};
    use mongodb::Database;

    pub fn new(db: Database) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        register(db)
    }

    pub fn register(db: Database) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path("register")
            .and(warp::post())
            .and(body::content_length_limit(1024 * 16))
            .and(body::json())
            .and(with_db(db))
            .and_then(register_handler)
    }

    fn with_db(db: Database) -> impl Filter<Extract=(Database, ), Error=std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}