use std::fmt::{Display, Formatter, Result};

pub mod user_repository;

pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(firstname: &str, lastname: &str, email: &str, password: &str) -> User {
        User {
            firstname: String::from(firstname),
            lastname: String::from(lastname),
            email: String::from(email),
            password: String::from(password),
        }
    }

    pub fn change_password(&self, new: &str) -> User {
        User {
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            email: self.email.clone(),
            password: String::from(new),
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ \
        \"firstname\": \"{}\", \
        \"lastname\": \"{}\", \
        \"email\": \"{}\", \
        \"password\": \"{}\" \
        }}", self.firstname, self.lastname, self.email, self.password)
    }
}
