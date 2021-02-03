use mongodb::bson::Document;

pub trait Entity {
    fn collection(&self) -> &str;
    fn convert_to_doc<T>(entity: T) -> Document;
    fn convert_to_entity<T>(document: Document) -> T;
}