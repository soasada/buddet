use mongodb::{bson::Document};

pub trait Entity {
    fn name(&self) -> &str;
    fn to_doc(&self) -> Document;
}