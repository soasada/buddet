use async_trait::async_trait;
use crate::user::user::User;
use crate::error::repository_error::RepositoryErrorKind;

#[async_trait]
pub trait UserRepository<T> {
    async fn save(&self, persistence: &T, user: User) -> Result<String, RepositoryErrorKind>;
    async fn find(&self, persistence: &T, id: &str) -> Option<User>;
}