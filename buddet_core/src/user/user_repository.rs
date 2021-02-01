use async_trait::async_trait;
use crate::user::user::User;

#[async_trait]
pub trait UserRepository<'a, T> {
    async fn save(&'a self, persistence: &T, user: User) -> Result<String, &'a str>;
    async fn find(&'a self, persistence: &T, id: &str) -> Result<User, &'a str>;
}