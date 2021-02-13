use buddet_db::database::mongo_database::MongoDatabase;

mod user;

#[tokio::main]
async fn main() {
    if let Ok(mdb) = MongoDatabase::new("mongodb://admin:password@localhost:27022/buddetdb", "buddetdb").await {
        let api = filters::new(&mdb);
        warp::serve(api)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
}

mod filters {
    use super::handlers;
    use buddet_db::database::mongo_database::MongoDatabase;
    use warp::{Filter, body};

    pub fn new<'a>(db: &'a MongoDatabase) -> impl Filter<Extract = impl warp::Reply + 'a, Error=warp::Rejection> + Clone {
        register(db.clone())
    }

    pub fn register<'a>(db: &'a MongoDatabase) -> impl Filter<Extract = impl warp::Reply + 'a, Error = warp::Rejection> + Clone {
        warp::path("register")
            .and(warp::post())
            .and(body::content_length_limit(1024 * 16))
            .and(body::json())
            .and(with_db(db))
            .and_then(handlers::register_handler)
    }

    fn with_db<'a>(db: &'a MongoDatabase) -> impl Filter<Extract = (&'a MongoDatabase,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use crate::user::create_user_request::CreateUserRequest;
    use buddet_db::database::mongo_database::MongoDatabase;
    use std::convert::Infallible;
    use buddet_core::user::User;
    use buddet_db::database::save;
    use buddet_db::user::UserEntity;
    use warp::http::StatusCode;

    pub async fn register_handler(request: CreateUserRequest, db: &MongoDatabase) -> Result<impl warp::Reply, Infallible> {
        let user = User::new(
            request.firstname.as_str(),
            request.lastname.as_str(),
            request.email.as_str(),
            request.password.as_str()
        );
        if let Ok(inserted) = save(db, UserEntity::from(user)).await {
            Ok(StatusCode::CREATED)
        } else {
            Ok(StatusCode::BAD_REQUEST)
        }
    }
}