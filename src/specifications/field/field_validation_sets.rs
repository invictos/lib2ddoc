#![allow(non_snake_case)]

use crate::specifications::{ErrorKind, Error};

pub fn validate_s_MAJ(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! c.is_uppercase() {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_CHIFFRES(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! c.is_numeric()  {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_SLASH(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == '/') {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_ESPACE(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == ' ') {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_TIRET(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == '-') {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_VIRGULE(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == ',') {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_HEXA(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! c.is_ascii_hexdigit() {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_POINT(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == '.')  {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_APOSTROPHE(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == '\'')  {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_BIERE(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        'O' | 'S' | 'H' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_BINAIRE(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        '0' | '1' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_PROPRIETAIRE(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        '1' | '2' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_DIPLOME(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        '4' | '5' | '6' | '7' | '8' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_GENRE(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        'M' | 'F' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_GENRE_AUTRE(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == 'X') {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_CONTRAT(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        '0' | '1' | '2' | '3' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }   
}
pub fn validate_s_GENRE_UNKNOWN(c: &char, _truncated: bool) -> Result<(), Error> {
    if ! (*c == 'U')  {
        return Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
    Ok(())
}
pub fn validate_s_PRELEVEMENT(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        'P' | 'N' | 'X' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_SEJOUR(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        '1' | '2' | '3' | '4' | '5' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}
pub fn validate_s_MENTIONS(c: &char, _truncated: bool) -> Result<(), Error> {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' => Ok(()),
        _ => Err(Error{
            kind: ErrorKind::NotInSet,
            value: String::from(""),
        })
    }
}