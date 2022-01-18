use std::collections::HashMap;

use crate::document::{Document, Headers, FieldZone, Field};
use crate::document::ErrorDocument;
use crate::specifications::{FieldSpecificationsStore, SpecificationsStore, DocumentSpecificationsStore};

#[derive(serde::Serialize)]
struct HeadersJSON<'a>{
    marqueur : &'a str,
    version : u8,
    identifiant_de_ac : &'a str,
    identifiant_du_certificat : &'a str,
    date_emission : &'a str,
    date_signature: &'a str,
    type_document: &'a str,
    type_document_description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    perimetre : Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pays_emetteur : Option<&'a str>,
}

impl <'a>From<&'a Headers> for HeadersJSON<'a> {
    fn from(headers: &'a Headers) -> HeadersJSON{
        let doc_spec = DocumentSpecificationsStore::get(&headers.type_document).expect("Can't happen because type is checked on decoding");

        HeadersJSON {
            marqueur : &headers.marqueur,
            version : headers.version.parse().expect("Version isn't integer"),
            identifiant_de_ac : &headers.identifiant_de_ac,
            identifiant_du_certificat : &headers.identifiant_du_certificat,
            date_emission : &headers.date_emission,
            date_signature: &headers.date_signature,
            type_document: &headers.type_document,
            type_document_description: &doc_spec.description,
            perimetre : headers.perimetre.as_deref(),
            pays_emetteur : headers.pays_emetteur.as_deref(),
        }
    }
}

#[derive(serde::Serialize)]
struct FieldJSON<'a>{
    name: &'a str,
    value: &'a str,
    is_valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>
}

impl <'a>From<&'a Field> for FieldJSON<'a> {
    fn from(field: &'a Field) -> Self {
        let field_spec = FieldSpecificationsStore::get(&field.id).expect("Impossible: Can't happen because id is checked on decoding");

        FieldJSON{
            name: field_spec.nom,
            value: &field.value,
            is_valid: field.is_valid(),
            errors: field.get_validity_error_strings(),
        }
    }
}


#[derive(serde::Serialize)]
struct DocumentJSON<'a>{
    is_ok: bool,
    headers: HeadersJSON<'a>,
    message: HashMap<&'a str, FieldJSON<'a>>,
    is_signature_valid: bool,
    annex: &'a str,
    is_document_valid: bool
}


impl DocumentJSON<'_> {
    fn create_message<'a>(fz: &'a FieldZone) -> HashMap<&'a str, FieldJSON>{
        fz.zone.iter()
        .map(|(k,v)| (k.as_str(), v.into()) )
        .collect()
    }
}

impl <'a>From<&'a Document> for DocumentJSON<'a> {
    fn from(document: &'a Document) -> Self {
        DocumentJSON { 
            is_ok: true,
            headers: (&document.decoded.headers).into(),
            message: Self::create_message(&document.decoded.message),
            is_signature_valid: document.signing.valid,
            annex: &document.raw.annex,
            is_document_valid: document.validity.valid
        }
    }
}


#[derive(serde::Serialize)]
struct ErrorJSON<'a>{
    is_ok: bool,
    kind: String,
    value: &'a str
}

impl <'a>From<&'a ErrorDocument> for ErrorJSON<'a>{
    fn from(error: &'a ErrorDocument) -> Self {
        ErrorJSON{
            is_ok: false,
            kind: error.kind.to_string(),
            value: &error.value
        }
    }
}

pub fn serialize_pretty(doc: Result<Document, ErrorDocument>) -> String{
    match doc {
        Ok(document) => serde_json::to_string_pretty(&DocumentJSON::from(&document)).expect("Conversion error document to documentJSON"),
        Err(error) => serde_json::to_string_pretty(&ErrorJSON::from(&error)).expect("Conversion error errordocument to errorJSON"),
    }
}

pub fn serialize(doc: Result<Document, ErrorDocument>) -> String{
    match doc {
        Ok(document) => serde_json::to_string(&DocumentJSON::from(&document)).expect("Conversion error document to documentJSON"),
        Err(error) => serde_json::to_string(&ErrorJSON::from(&error)).expect("Conversion error errordocument to errorJSON"),
    }
}