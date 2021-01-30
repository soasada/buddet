use crate::user::User;

pub trait UserRepository {
    fn save(&self, user: User) -> String;
}