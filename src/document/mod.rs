pub use self::{decoded::{DocumentDecoded, builder::DocumentDecodedBuilder, field_zone::{Field, FieldZone}, headers::Headers}, raw::DocumentRaw, signing::Signing, validity::Validity};

mod raw;
mod signing;
mod validity;
mod decoded;
mod error;

use error::{Error, ErrorKind};
pub use error::Error as ErrorDocument;
pub use error::ErrorKind as ErrorDocumentKind;

use error::ToError;
pub struct Document{
    pub raw: DocumentRaw,
    pub decoded: DocumentDecoded,
    pub signing: Signing,
    pub validity: Validity
}

pub fn decode(str: &str) -> Result<Document, Error>{

    if str.len() == 0 {
        return Err(Error {
            kind: ErrorKind::NoMatrix,
            value: String::from("No matrix found"),
        })
    }
    
    let (headers, headers_size) = Headers::new(str);

    let document_raw = DocumentRaw::new(str, headers_size)?;

    let mut document_decoded = DocumentDecodedBuilder::new()
                                .add_headers(headers)
                                .decode_message(&document_raw.message)?
                                .collect().to_error(ErrorKind::Decoder, String::from("Error decoding"))?;

    let signing = Signing::verify_signature(&document_raw, &document_decoded)?;

    let validity = Validity::validate(&mut document_decoded);

    Ok(Document { 
        raw: document_raw,
        decoded: document_decoded,
        signing,
        validity
    })
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn document_decode(){
        let message = "DC04FR000001198519D31201FR90MAITRE/SPECIMEN/NATACHA92RAISON SOCIALE DE TEST94SAISIE CONSERVATOIRE DE CREANCES962111201791MME/BERTHIER/CORINNE93RAISON SOCIALE DU TIERS CONCERNE951896547853AB0CNB2WS43TNFSXELLKOVZXI2LDMUXGM4RPGE4DSNRVGQ3TQNJTIFBAOOXND3NRRGDZKBYZ6VDMHSM7WHJ323ICLGTSEELJ74OW3E4GGFYI3GX6IXCN4HF45JWYZKHHU7GXTBMCSHOSU5GOUHJYN4PIH6VAA2Q";
        let document = decode(message).expect("Impossible: cannot happen in test");

        assert_eq!(document.decoded.headers.marqueur, "DC");
        assert_eq!(document.decoded.headers.version, "04");
        assert_eq!(document.decoded.headers.identifiant_de_ac, "FR00");
        assert_eq!(document.decoded.headers.identifiant_du_certificat, "0001");
        assert_eq!(document.decoded.headers.date_emission, "1985");
        assert_eq!(document.decoded.headers.date_signature, "19D3");
        assert_eq!(document.decoded.headers.type_document, "12");
        assert_eq!(document.decoded.headers.perimetre.expect("Impossible car version 4"), "01");
        assert_eq!(document.decoded.headers.pays_emetteur.expect("Impossible car version 4"), "FR");

        let field = document.decoded.message.get("90".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "MAITRE/SPECIMEN/NATACHA");
        
        let field = document.decoded.message.get("92".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "RAISON SOCIALE DE TEST");
        
        let field = document.decoded.message.get("94".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "SAISIE CONSERVATOIRE DE CREANCES");
        
        let field = document.decoded.message.get("96".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "21112017");
        
        let field = document.decoded.message.get("91".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "MME/BERTHIER/CORINNE");
        
        let field = document.decoded.message.get("93".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "RAISON SOCIALE DU TIERS CONCERNE");
        
        let field = document.decoded.message.get("95".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "1896547853AB");
        
        let field = document.decoded.message.get("0C".to_string()).expect("Impossible: cannot happen in test");
        assert_eq!(field.value, "NB2WS43TNFSXELLKOVZXI2LDMUXGM4RPGE4DSNRVGQ3TQNJTIFBA");

        assert!(document.validity.valid);
        
    }
}