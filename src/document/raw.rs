use ascii::AsciiChar;

use super::error::{Error, ErrorKind, ToError};

pub struct DocumentRaw{
    pub data_zone: String,
    pub headers: String,
    pub message: String,
    pub signature: String,
    pub annex: String
}

impl DocumentRaw{
    pub fn new(all: &str, header_size: usize) -> Result<DocumentRaw, Error>{
        let us_pos = all.find(AsciiChar::US.as_char()).to_error(ErrorKind::Decoder, String::from("Cant find US"))?;

        Ok(DocumentRaw{
            data_zone: all[0..us_pos].to_string(),
            headers: all[0..header_size].to_string(),
            message: all[header_size..us_pos].to_string(),
            signature: all[us_pos+1..].to_string(),
            annex: "".to_string()
        })

    }
}

    #[test]
    fn document_raw(){
        let dr = DocumentRaw::new(
            "DC04FR0000011E581E58B201FRF0DEPOTHPRIMF1NEGF201011955F3FF4945006F5NF6070420211010MRZICP23SZTCLV2APC5P2AQVZDKHDKRG5KOIYGBKDXUA7N5O75QLWMKJBFSL3WJZYG2HHF4Z3K5E6E2GJFOVSV7E2W2XVHEXXVDXYNY",
            26
        ).expect("Impossible: cannot happen in test");

        assert_eq!(dr.headers, "DC04FR0000011E581E58B201FR");
        assert_eq!(dr.message, "F0DEPOTHPRIMF1NEGF201011955F3FF4945006F5NF6070420211010");
        assert_eq!(dr.signature, "MRZICP23SZTCLV2APC5P2AQVZDKHDKRG5KOIYGBKDXUA7N5O75QLWMKJBFSL3WJZYG2HHF4Z3K5E6E2GJFOVSV7E2W2XVHEXXVDXYNY");
        assert_eq!(dr.annex, "");
    }