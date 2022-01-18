mod check_signature_http;
mod test_data;


/*
    #################
    Security module !
    #################

    This module is in charge of handling everything releated to the signature,
    The check_signature is the only interface, feel free to implement it as needed.

    Testing data is not distributed for legal reasons
*/
pub fn check_signature(data_zone: &str, signature: &str, authority_id: &str, certificate_id: &str) -> Result<bool, String>{
    check_signature_http::check_signature(data_zone, signature, authority_id, certificate_id)
}