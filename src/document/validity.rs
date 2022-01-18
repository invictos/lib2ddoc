use crate::specifications::{DocumentSpecificationsStore, FieldSpecificationsStore, SpecificationsStore};

use super::decoded::{DocumentDecoded, field_zone::FieldZone};

pub struct Validity{
    pub valid: bool
}

impl Validity{
    
    pub fn validate(document: &mut DocumentDecoded) -> Validity{

        //Validate section 7
        let fz_valid = is_field_zone_valid(&mut document.message);
        
        //Validate section 8
        let doc_valid = is_document_valid(&document);

        Validity{
            valid: fz_valid && doc_valid
        }
    }
}

fn is_document_valid(doc: &DocumentDecoded) -> bool {
    let spec = DocumentSpecificationsStore::get(&doc.headers.type_document).expect("Impossible: tested when parsing headers");

    let a: Vec<&str> = doc.message.zone.keys()
        .map(|k| k.as_str())
        .collect();


    spec.validate(a.as_slice()).is_ok()
}

fn is_field_zone_valid(fz: &mut FieldZone) -> bool{
    let mut is_valid = true;

    for (key, field) in fz.zone.iter_mut() {
        let validity = FieldSpecificationsStore
            ::get(key).expect("Impossible: key checked when decoding")
            .validate(&field.value, field.is_truncated);

        match validity {
            Ok(_) => field.set_validity_ok(),
            Err(e) => {
                field.set_validity_err(e);

                is_valid = false;
            }
        }
    }

    is_valid
}