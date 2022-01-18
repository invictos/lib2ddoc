use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    General,
    NotInSet,
    FailedFunction,
    SizeNotRespected,
    InvalidCaracter,
    DocumentFields
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
        write!(f, "[{}] {}", self.kind, self.value)
    }
}