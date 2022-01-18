use crate::{security::check_signature};

use super::{decoded::DocumentDecoded, error::{Error, ErrorKind}, raw::DocumentRaw};

pub struct Signing{
    pub valid: bool
}

impl Signing{
    pub fn verify_signature(raw: &DocumentRaw, decoded: &DocumentDecoded) -> Result<Signing, Error>{

        let is_valid = check_signature(
                &raw.data_zone,
                &raw.signature, 
                &decoded.headers.identifiant_de_ac, 
                &decoded.headers.identifiant_du_certificat)
            .map_err(|err| Error {
                kind: ErrorKind::Signing,
                value: err,
            })?;

        Ok(Signing{
            valid: is_valid
        })
    }
}