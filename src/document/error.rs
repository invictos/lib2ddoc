use std::fmt::{self, Display};
use core::fmt::Debug;
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    General,
    Impossible,
    NoMatrix,
    FieldZoneDecoder,
    Decoder,
    Signing
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct Error{
    pub kind: ErrorKind,
    pub value: String
}

impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]{}", self.kind as u32, self.value)
    }
}
pub trait ToError<T> {
    fn to_error(self, kind: ErrorKind, value: String) -> Result<T, Error>;
}

impl<T>  ToError<T>  for Option<T> {
    fn to_error(self, kind: ErrorKind, value: String) -> Result<T, Error>{
        if self.is_some(){
            return Ok(self.expect("Impossible: self.is_some() verified"))
        }   
        Err(Error{
            kind,
            value,
        })
    }
}

impl<T,E>  ToError<T> for  Result<T, E> 
where 
    E:Debug
{
    fn to_error(self, ek: ErrorKind, mes: String) -> Result<T, Error>{
        if self.is_ok(){
            return Ok(self.expect("Impossible: self.is_ok() verified"))
        }   
        Err(Error{
            kind: ek,
            value: mes,
        })
    }
}