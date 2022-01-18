use std::collections::HashMap;
use ascii::AsciiChar;


use crate::{document::error::{Error, ErrorKind, ToError}, specifications::{FieldSpecificationsStore, SpecificationsStore}};

use super::{Field, FieldZone};
use std::convert::TryFrom;

fn consume<'a>(str: &'a mut &str, nb: usize, offset: usize) -> Option<&'a str>{
    /*
        returns a reference to the nb first chars of str
        and shift str from nb+offset chars
    */
    let r = str.get(..nb)?;
    *str = str.get(nb+offset..)?;

    Some(r)
}

pub fn decode(str: &str) -> Result<FieldZone, Error> {
    let mut fz= FieldZone { 
        zone:HashMap::new()
    };

    let mut msg = str;

    while msg.len() != 0 {
        let mut is_truncated = false;

        let id = consume(&mut msg, 2, 0).to_error(ErrorKind::FieldZoneDecoder, String::from("Cant parse the id"))?.to_string();

        let field_specification = FieldSpecificationsStore::get(&id).to_error(ErrorKind::FieldZoneDecoder, format!("cant find field id {}",id))?;
        
        let field_size_min: usize = usize::try_from(field_specification.taille_min).to_error(ErrorKind::FieldZoneDecoder, format!("usize too small for i16 {}",field_specification.taille_min))?;

        let field_value = if field_specification.is_size_fixed() {
            //We have a field with a fixed size
            consume(&mut msg, field_size_min, 0).to_error(ErrorKind::FieldZoneDecoder, String::from("Expected fixed size value,god end if message"))?.to_string()

        }else{
            let msg_len = msg.len();

            let field_size_max: usize = if field_specification.taille_max.is_negative() {
                msg_len //Field has no maximum size, thus at most length(msg)
            }else{
                usize::try_from(field_specification.taille_max).to_error(ErrorKind::FieldZoneDecoder, format!("usize too small for i16 {}",field_specification.taille_max))?
            };
            
            //We have a field with a variable size
            match msg[0..(field_size_max+1).min(msg_len)].find(AsciiChar::RS.as_char()) {
                Some(adress_rs) => {
                    is_truncated = true;
                    consume(&mut msg, adress_rs, 1).to_error(ErrorKind::Impossible, String::from("Impossible"))?.to_string()
                }, 
                None => match msg[0..(field_size_max+1).min(msg_len)].find(AsciiChar::GS.as_char()){
                    Some(adress_gs) => {
                        
                        consume(&mut msg, adress_gs, 1).to_error(ErrorKind::Impossible, String::from("Impossible"))?.to_string()
                    },
                    None => {
                        //Couldn't find rs/gs in a variable field, eigther:
                        // last field is a variable field: consume what's left
                        // OR
                        // field size is too big: error
                        if msg_len < field_size_max {
                            consume(&mut msg, msg_len, 0).to_error(ErrorKind::Impossible, String::from("Impossible"))?.to_string()
                        }else{

                            return Err(Error{
                                kind: ErrorKind::FieldZoneDecoder,
                                value: String::from("Field is too big !"),
                            })
                        }
                    },
                },
            }
        };

        fz.insert(id.to_string(), Field {
            id,
            value: field_value,
            is_truncated,
            validity: None //Validity is checked later
        });
    }

    Ok(fz)
}

#[cfg(test)]
mod tests{
    use super::*; 

    #[test]
    fn test_message1(){
        let mes="26FR247500010MME/SPECIMEN/NATACHA22145 AVENUE DES SPECIMENS";

        let fz = decode(mes).expect("Impossible: cannot happen in test");
        assert_eq!(fz.zone.contains_key("26"),true);
        assert_eq!(fz.zone.contains_key("10"),true);
        assert_eq!(fz.zone.contains_key("22"),true);
        assert_eq!(fz.zone.contains_key("24"),true);
        assert_eq!(fz.get("26".to_string()).expect("Impossible: cannot happen in test").value,"FR");
        assert_eq!(fz.get("10".to_string()).expect("Impossible: cannot happen in test").value,"MME/SPECIMEN/NATACHA");
        assert_eq!(fz.get("22".to_string()).expect("Impossible: cannot happen in test").value,"145 AVENUE DES SPECIMENS");
        assert_eq!(fz.get("24".to_string()).expect("Impossible: cannot happen in test").value,"75000");
    }
}