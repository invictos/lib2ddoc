

use crate::document::error::{Error, ErrorKind, ToError};

use super::{DocumentDecoded, field_zone::{self, FieldZone}, headers::Headers};

pub struct DocumentDecodedBuilder{
    headers: Option<Headers>,
    message: Option<FieldZone>
}


impl DocumentDecodedBuilder{
    pub fn new() -> DocumentDecodedBuilder{
        DocumentDecodedBuilder{
            headers: None,
            message: None
        }
    }

    pub fn collect(self) -> Result<DocumentDecoded, Error>{
        if self.headers.is_none() || self.message.is_none() {
            panic!("Must fill struct");
        }

        Ok(DocumentDecoded{
            headers: self.headers.to_error(ErrorKind::Decoder, String::from("Impossible"))?,
            message: self.message.to_error(ErrorKind::Decoder, String::from("Impossible"))?,
        })
    }

    pub fn add_headers(mut self, headers: Headers) -> DocumentDecodedBuilder{
        self.headers = Some(headers);
        
        self
    }

    pub fn decode_message(mut self, message: &str) -> Result<DocumentDecodedBuilder, Error>{
        self.message = Some(field_zone::decoder::decode(message)?);

        Ok(self)
    }
}