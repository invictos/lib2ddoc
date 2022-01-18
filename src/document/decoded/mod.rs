pub mod headers;
pub mod builder;
pub mod field_zone;

pub struct DocumentDecoded{
    pub headers: headers::Headers,
    pub message: field_zone::FieldZone,
}