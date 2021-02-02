use async_trait::async_trait;
use crate::repository::Repository;
use crate::user::User;

#[async_trait]
pub trait UserRepository: Repository<User> {}