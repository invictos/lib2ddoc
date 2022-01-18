use base32;
use reqwest::StatusCode;
use x509_parser::parse_x509_certificate;
use p256::{PublicKey, ecdsa::{VerifyingKey, signature::{
            Signature,
            Verifier
        }}, pkcs8::FromPublicKey
    };

#[cfg(debug_assertions)]
use super::test_data;


fn decode_signature(signature: &str) -> Result<Vec<u8>, String> {
    base32::decode(
        base32::Alphabet::RFC4648 { padding: true },
        signature
    ).ok_or_else(|| String::from("Signature is not base32"))
}

fn fetch_certificate(authority_id: &str, certificate_id: &str) -> Result<Vec<u8>, String>{
    // Returns a DER certificate

    #[cfg(debug_assertions)]
    if authority_id == "FR00" {
        if certificate_id == "0001" {
            return Ok(test_data::test_cert_fr00_0001())
        }
        return Err(String::from("Unknown test certificate"))
    }


    let url = match authority_id {
        "FR01" => "http://cert.pki-2ddoc.ariadnext.fr/pki-2ddoc.der",
        "FR02" => return Err(String::from("FR02 authority is revoked")),
        "FR03" => "http://certificates.certigna.fr/search.php",
        "FR04" => "http://pki-g2.ariadnext.fr/pki-2ddoc.der",
        "FR05" => "http://cert.cev.ants.gouv.fr/search.php",
        _      => return Err(format!("Authority does not exist: {}", authority_id))
    };

    let url = format!("{}?name={}", url, certificate_id);

    let res = reqwest::blocking::get(url)
        .map_err(|_| String::from("Request failed, no internet ?"))?;
    
    match res.status() {
        StatusCode::NO_CONTENT => Err(format!("Certificate does not exist for authority: {}/{}", authority_id, certificate_id)),
        StatusCode::OK         => Ok(res.bytes().expect("Can't convert to bytes").to_vec()),
        _                      => Err(String::from("Failed to contact authority"))
    }
}

pub fn check_signature(data_zone: &str, signature: &str, authority_id: &str, certificate_id: &str) -> Result<bool, String>{

    let signature = decode_signature(signature)?;

    let certificate = fetch_certificate(authority_id, certificate_id)?;

    let (_, certificate) = parse_x509_certificate(&certificate)
        .map_err(|_| String::from("Can't decode X509 certificate"))?;

    let public_key = certificate.public_key().raw;

    let public_key: VerifyingKey = PublicKey::from_public_key_der(&public_key)
        .map_err(|_| String::from("Can't parse public key"))?
        .into();

    //Create the signature struct for p256, signature is the one from QR code
    let signature = Signature::from_bytes(signature.as_slice())
        .map_err(|_| String::from("Can't parse signature"))?;

    //Do the verification on the public key, data against signature.
    let is_valid = public_key.verify(data_zone.as_bytes(), &signature).is_ok();

    Ok(is_valid)
}



#[cfg(test)]
mod test{
    use std::str::from_utf8;
    use ascii::AsciiChar;
    use super::*;

    #[test]
    fn check_signature_local(){
        let message_raw = test_data::test_message_fr00_0000();
        let message= from_utf8(&message_raw).expect("Impossible: cannot happen in test");

        let (data, signature_base32) = message.split_once(AsciiChar::US.as_char()).expect("No US separator found");
        
        let verified = check_signature(data, signature_base32, "FR00", "0001").expect("Impossible: cannot happen in test");
    
        assert!(verified);
    }

    #[test]
    fn check_signature_remote(){
        let message_raw = test_data::test_message_fr03_av01();
        let message= from_utf8(&message_raw).expect("Impossible: cannot happen in test");
    
        let (data, signature_base32) = message.split_once(AsciiChar::US.as_char()).expect("No US separator found");
        
        let verified = check_signature(data, signature_base32, "FR03", "AV01").expect("Impossible: cannot happen in test");
    
        assert!(verified);
    }
}