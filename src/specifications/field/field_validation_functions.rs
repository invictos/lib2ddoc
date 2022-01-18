#![allow(non_snake_case)]
use std::str::FromStr;

use crate::specifications::{Error, ErrorKind};

pub fn parse<T: FromStr>(value: &str, err_message: String) -> Result<T, Error>{
    value.parse().map_err(|_| Error{
        kind: ErrorKind::InvalidCaracter,
        value: err_message,
    })
}

pub fn validate_f_PROCEDURE(str: &str, _truncated: bool) -> Result<(), Error>{
    match str {
        "PN"|"PA"|"PD" => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from("Procedure not valid"),
        })
    }
}

pub fn validate_f_REGIONS(str: &str, _truncated: bool) -> Result<(), Error>{
    match str {
        "KO"|"AR"|"BF"|"BR"|"CV"|"GE"|"HF"|"IF"|"NO"|"NA"|"OC"|"PL"|"PA" => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from("Region not valid"),
        })
    }
}

pub fn validate_f_DIPLOME_TYPE(str: &str, _truncated: bool) -> Result<(), Error>{
    match str {
        "BR"|"CA"|"BE"|"BA"|"BP"|"BS"|"BT"|"DU"|"LC"|"LP"|"DE"|"MA"|"MB"|"IN"|"DR" => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from("Diploma type not valid"),
        })
    }
}

pub fn validate_f_PIECE_IDENTITE_TYPE(str: &str, _truncated: bool) -> Result<(), Error>{
    match str {
        "ID" | "IR" | "D" => Ok(()),
        _ => {
            if str.len() != 2 || !(str.starts_with('V') || str.starts_with('P')) {
                return Err(Error{
                    kind: ErrorKind::FailedFunction,
                    value: String::from("Identity type invalid"),
                })
            }
            Ok(())
        }
    }
}
pub fn validate_f_HHMMSS(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 6{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
    let hh=parse::<i32>(&str[0..2], format!("Cannot convert hh ({}) to integer", str[0..2].to_string()))?;
    if hh<=0 || hh>23  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The hour is not valid"),
        })
    }
    let mm=parse::<i32>(&str[2..4], format!("Cannot convert mm ({}) to integer", str[2..4].to_string()))?;
    if mm<=0 || mm>59  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The minutes are not valid"),
        })
    }
    let ss=parse::<i32>(&str[4..6], format!("Cannot convert ss ({}) to integer", str[4..6].to_string()))?;
    if ss<=0 || ss>59  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The seconds are not valid"),
        })
    }

    Ok(())
}

pub fn validate_f_HEURE(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 4{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
    let hh=parse::<i32>(&str[0..2], format!("Cannot convert hh ({}) to integer", str[0..2].to_string()))?;
    if hh<=0 || hh>23  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The hour is not valid"),
        })
    }
    let mm=parse::<i32>(&str[2..4], format!("Cannot convert mm ({}) to integer", str[2..4].to_string()))?;
    if mm<=0 || mm>59  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The minutes are not valid"),
        })
    }

    Ok(())
}

pub fn validate_f_AAAAMMJJ(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len() != 8 {
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }

    let aaaa=parse::<i32>(&str[0..4], format!("Cannot convert mm ({}) to integer", str[0..4].to_string()))?;
    if aaaa<=1900 || aaaa>2022  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The year is not valid"),
        })
    }

    let mois=parse::<i32>(&str[4..6], format!("Cannot convert MM ({}) to integer", str[4..6].to_string()))?;
    if mois<=0 || mois>12 {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The months are not valid"),
        })
    }

    let jj=parse::<i32>(&str[6..8], format!("Cannot convert jj ({}) to integer", str[6..8].to_string()))?;
    if jj<=1 || jj>31  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The day doesnt exist"),
        })
    }

    Ok(())
}


pub fn validate_f_DATEHEURE(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 12{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
            
    let jj=parse::<i32>(&str[0..2], format!("Cannot convert jj ({}) to integer", str[0..2].to_string()))?;
    if jj<=1 || jj>31  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The day doesnt exist"),
        })
    }
    let mois=parse::<i32>(&str[2..4], format!("Cannot convert MM ({}) to integer", str[2..4].to_string()))?;
    if mois<=0 || mois>12 {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The months are not valid"),
        })
    }
    let aaaa=parse::<i32>(&str[4..8], format!("Cannot convert mm ({}) to integer", str[4..8].to_string()))?;
    if aaaa<=1900 || aaaa>2022  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The year is not valid"),
        })
    }
    let hh=parse::<i32>(&str[8..10], format!("Cannot convert hh ({}) to integer", str[8..10].to_string()))?;
    if hh<=0 || hh>23  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The hour is not valid"),
        })
    }
    let mm=parse::<i32>(&str[10..12], format!("Cannot convert mm ({}) to integer", str[10..12].to_string()))?;
    if mm<=0 || mm>59  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The minutes are not valid"),
        })
    }
    Ok(())

}
pub fn validate_f_URL(str: &str, truncated: bool) -> Result<(), Error>{
    if truncated{
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The URL must not be truncated"),
        })
    }  
    let str_raw = base32::decode(
        base32::Alphabet::RFC4648 { padding: true },
        str
    );
    let verifie=str_raw.is_some();
    if !verifie{
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The field must be in base32"),
        })
    }
    Ok(())
}
pub fn validate_f_LOINC(_str: &str, _truncated: bool) -> Result<(), Error>{
    Ok(())
}

pub fn validate_f_DATE(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 8{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
            
    let jj=parse::<i32>(&str[0..2], format!("Cannot convert jj ({}) to integer", str[0..2].to_string()))?;
    if jj<=0 || jj>31  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The day doesnt exist"),
        })
    }
    let mois=parse::<i32>(&str[2..4], format!("Cannot convert MM ({}) to integer", str[2..4].to_string()))?;
    if mois<=0 || mois>12 {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The months are not valid"),
        })
    }
    let aaaa=parse::<i32>(&str[4..8], format!("Cannot convert mm ({}) to integer", str[4..8].to_string()))?;
    if aaaa<=1900 || aaaa>2021  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The year is not valid"),
        })
    }
    Ok(())
}

pub fn validate_f_DATE_INCONNUE(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 8{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
            
    let jj=parse::<i32>(&str[0..2], format!("Cannot convert jj ({}) to integer", str[0..2].to_string()))?;
    if jj<0 || jj>31  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The day doesnt exist"),
        })
    }
    let mois=parse::<i32>(&str[2..4], format!("Cannot convert MM ({}) to integer", str[2..4].to_string()))?;
    if mois<0 || mois>12 {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The months are not valid"),
        })
    }
    let aaaa=parse::<i32>(&str[4..8], format!("Cannot convert mm ({}) to integer", str[4..8].to_string()))?;
    if aaaa<=1900 || aaaa>2021  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The year is not valid"),
        })
    }
    Ok(())
}

pub fn validate_f_DATE_LUNAIRE(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 8{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
            
    let jj=parse::<i32>(&str[0..2], format!("Cannot convert jj ({}) to integer", str[0..2].to_string()))?;
    if jj<0 || jj>30  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The day doesnt exist"),
        })
    }
    let mois=parse::<i32>(&str[2..4], format!("Cannot convert MM ({}) to integer", str[2..4].to_string()))?;
    if mois<0 || mois>50 {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The months are not valid"),
        })
    }
    let aaaa=parse::<i32>(&str[4..8], format!("Cannot convert mm ({}) to integer", str[4..8].to_string()))?;
    if aaaa<=1900 || aaaa>2021  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("The year is not valid"),
        })
    }
    Ok(())
}

pub fn validate_f_CLASSE_VEHICULE(str: &str, _truncated: bool) -> Result<(), Error>{
    if str.len()!= 3{
        return Err(Error{
            kind: ErrorKind::SizeNotRespected,
            value: String::from("The size of the field is not respected"),
        })
    }
    let value1=parse::<i32>(&str[0..1], format!("Cannot convert hh ({}) to integer", str[0..1].to_string()))?;
    if value1<0 || value1>1  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("Value1 is not valid"),
        })
    }
    let value2=parse::<i32>(&str[1..2], format!("Cannot convert mm ({}) to integer", str[1..2].to_string()))?;
    if value2<=0 || value2>6  {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("value2 is not valid"),
        })
    }
    let value3=parse::<i32>(&str[2..3], format!("Cannot convert ss ({}) to integer", str[2..3].to_string()))?;
    if value3 !=0 {
        return Err(Error{
            kind: ErrorKind::FailedFunction,
            value: String::from("value3 is not valid"),
        })
    }

    Ok(())
}
pub fn validate_f_PERMIS_TYPE(str: &str, _truncated: bool) -> Result<(), Error>{
    match str {
        "RII"|"RIR"|"RRP"|"APC" => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from("Licence type not valid"),
        })
    }
}
pub fn validate_f_PERMIS_CATEGORIE(str: &str, _truncated: bool) -> Result<(), Error>{
    let all_ok = str.split_ascii_whitespace().all(|permis| match permis {
        "AM" | "A1" | "A2" | "A" | "B1" | "B" | "C1" | "C" | "D1" | "D" | "BE" | "C1E" | "CE" | "D1E" | "DE" => true,
        _ => false
    });

    if ! all_ok {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from("Licence category not valid"),
        }) 
    }

    Ok(())
}
pub fn validate_f_PERMIS_ARRETE(str: &str, _truncated: bool) -> Result<(), Error>{
    match str {
        "1A" | "1B" | "1E" | "1F" | "2E" | "2F" | "3A" | "3B" | "3E" | "3F" | "3I" | "43" | "44" | "47" | "4X" => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from("Type not valid"),
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*; 

    #[test]
    fn validate_f_PROCEDURE_ok(){
        assert!(validate_f_PROCEDURE("PN", false).is_ok());
    } 

    #[test]
    fn validate_f_PROCEDURE_error(){
        assert!(validate_f_PROCEDURE("PK", false).is_err());
    } 

    #[test]
    fn validate_f_REGIONS_ok(){
        assert!(validate_f_REGIONS("KO", false).is_ok());
    } 

    #[test]
    fn validate_f_REGIONS_error(){
        assert!(validate_f_REGIONS("KK", false).is_err());
    } 

    #[test]
    fn validate_f_DIPLOME_TYPE_ok(){
        assert!(validate_f_DIPLOME_TYPE("BR", false).is_ok());
    } 

    #[test]
    fn validate_f_DIPLOME_TYPE_error(){
        assert!(validate_f_DIPLOME_TYPE("BB", false).is_err());
    } 

    #[test]
    fn validate_f_PIECE_IDENTITE_TYPE_ok(){
        assert!(validate_f_PIECE_IDENTITE_TYPE("VX", false).is_ok());
    } 

    #[test]
    fn validate_f_PIECE_IDENTITE_TYPE_error(){
        assert!(validate_f_PIECE_IDENTITE_TYPE("DP", false).is_err());
    } 

    #[test]
    fn validate_f_hhmmss_ok(){
        let str_test="121350";
        let resultat =validate_f_HHMMSS(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_hhmmss_error(){
        let str_test="897854";
        let resultat =validate_f_HHMMSS(str_test,false).is_err();
        assert!(resultat);
    }

    #[test]
    fn validate_f_HEURE_ok(){
        let str_test="1213";
        let resultat =validate_f_HEURE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_HEURE_error(){
        let str_test="8978";
        let resultat =validate_f_HEURE(str_test,false).is_err();
        assert!(resultat);
    }

    #[test]
    fn validate_f_datetime_ok(){
        let str_test="050220200520";
        let resultat =validate_f_DATEHEURE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_datetime_error(){
        let str_test="05022020502x";
        let resultat =validate_f_DATEHEURE(str_test,false).is_err();
        assert!(resultat);
    }

    #[test]
    fn validate_f_AAAAMMJJ_ok(){
        let str_test="20211127";
        let resultat =validate_f_AAAAMMJJ(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_AAAAMMJJ_error(){
        let str_test="20211132";
        let resultat =validate_f_AAAAMMJJ(str_test,false).is_err();
        assert!(resultat);
    }

    #[test]
    fn validate_f_url_ok(){
        let str_test="NB2WS43TNFSXELLKOVZXI2LDMUXGM4RPGE4DSNRVGQ3TQNJTIFBA";
        let resultat =validate_f_URL(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_url_error(){
        let str_test="https://www.google.com/";
        let resultat =validate_f_URL(str_test,false).is_err();
        assert!(resultat);
    }
    #[test]
    fn validate_f_loinc_ok(){
        let str_test="https://www.google.com/";
        let resultat =validate_f_LOINC(str_test,false).is_ok();
        assert!(resultat);
    }

    #[test]
    fn validate_f_date_ok(){
        let str_test="15122020";
        let resultat =validate_f_DATE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_date_error(){
        let str_test="05002020";
        let resultat =validate_f_DATE(str_test,false).is_err();
        assert!(resultat);
    }

    #[test]
    fn validate_f_date_inconue_ok(){
        let str_test="00002020";
        let resultat =validate_f_DATE_INCONNUE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_date_inconnue_error(){
        let str_test="0000020";
        let resultat =validate_f_DATE_INCONNUE(str_test,false).is_err();
        assert!(resultat);
    }

    #[test]
    fn validate_f_vehicules_ok(){
        let str_test="150";
        let resultat =validate_f_CLASSE_VEHICULE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_vehicules_error(){
        let str_test="151";
        let resultat =validate_f_CLASSE_VEHICULE(str_test,false).is_err();
        assert!(resultat);
    }
    #[test]
    fn validate_f_permis_type_ok(){
        let str_test="RIR";
        let resultat =validate_f_PERMIS_TYPE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_permis_type_error(){
        let str_test="RRR";
        let resultat =validate_f_PERMIS_TYPE(str_test,false).is_err();
        assert!(resultat);
    }
    #[test]
    fn validate_f_permis_categorie_ok(){
        let str_test="AM B BE DE";
        let resultat =validate_f_PERMIS_CATEGORIE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_permis_categorie_error(){
        let str_test="AM A1 C G";
        let resultat =validate_f_PERMIS_CATEGORIE(str_test,false).is_err();
        assert!(resultat);
    }
    #[test]
    fn validate_f_permis_arrete_ok(){
        let str_test="4X";
        let resultat =validate_f_PERMIS_ARRETE(str_test,false).is_ok();
        assert!(resultat);
    }
    #[test]
    fn validate_f_permis_arrete_error(){
        let str_test="6B";
        let resultat =validate_f_PERMIS_ARRETE(str_test,false).is_err();
        assert!(resultat);
    }
}