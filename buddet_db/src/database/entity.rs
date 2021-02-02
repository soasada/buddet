use mongodb::bson::Document;

pub trait Entity {
    fn collection(&self) -> &str;
    fn convert_to_doc(&self) -> Document;
}