use std::fmt::{Display, Formatter, Result};

pub struct User {
    firstname: &'static str,
    lastname: &'static str,
    email: &'static str,
    password: &'static str,
}

impl User {
    pub fn new(firstname: &'static str, lastname: &'static str, email: &'static str, password: &'static str) -> User {
        User { firstname, lastname, email, password }
    }

    pub fn changePassword(&self, new: &'static str) -> User {
        User { firstname: self.firstname, lastname: self.lastname, email: self.email, password: new }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ \"firstname\": {}, \"lastname\": {}, \"password\": {} }}", self.firstname, self.lastname, self.password)
    }
}