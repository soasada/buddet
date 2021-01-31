use crate::user::user::User;

pub trait UserRepository<T> {
    fn save(&self, persistence: &T, user: User) -> Result<String, &str>;
    fn find(&self, persistence: &T, id: &str) -> Result<User, &str>;
}