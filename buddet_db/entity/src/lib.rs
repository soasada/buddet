use mongodb::bson::Document;

pub trait Entity {
    type ToEntity;

    fn collection() -> &'static str;
    fn convert_to_doc(&self) -> Document;
    fn convert_to_entity(document: Document) -> Self::ToEntity;
}