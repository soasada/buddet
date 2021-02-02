use async_trait::async_trait;
use crate::repository::repository_error::RepositoryErrorKind;

pub mod repository_error;

#[async_trait]
pub trait Repository<T> {
    async fn save(&self, entity: T) -> Result<String, RepositoryErrorKind>;
    async fn update(&self, entity: T, id: &str) -> Result<String, RepositoryErrorKind>;
    async fn delete(&self, id: &str) -> Result<String, RepositoryErrorKind>;
    async fn find(&self, id: &str) -> Option<T>;
    async fn find_all(&self) -> Vec<T>;
}