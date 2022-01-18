use crate::specifications::{Error, ErrorKind, Specification};

pub struct FieldSpecification{
    pub id: &'static str,
    pub nom: &'static str,
    pub taille_min: i16,
    pub taille_max: i16,
    pub type_identifiant: &'static str,
    pub description: &'static str,
    pub validate: ValidatingConditionField,
}

impl FieldSpecification{
    pub fn validate(&self, value: &str, truncated: bool) -> Result<(), Vec<Error>> {
        match &self.validate {
            ValidatingConditionField::FUNCTION(functions) => validate_functions(functions, value, truncated),
            ValidatingConditionField::SET(conditions) => validate_sets(conditions, value, truncated),
        }
    }
    pub fn is_size_fixed(&self) -> bool{
        self.taille_min == self.taille_max
    }
}

fn validate_functions(functions: &[fn(&str, bool) -> Result<(), Error>], value: &str, truncated: bool) -> Result<(), Vec<Error>>{

    let errors: Vec<Error> = functions.iter()
        .filter_map(|f| f(value, truncated).err())
        .collect();

    if !errors.is_empty() {
        return Err(errors)
    }

    Ok(())
}

fn validate_sets(conditions: &[fn(&char, bool) -> Result<(), Error>], value: &str, truncated: bool) -> Result<(), Vec<Error>>{
    let mut errors = Vec::new();

    for (i, c) in value.chars().enumerate() {
        let char_is_in_one_set = conditions.iter()
            .any(|f| f(&c, truncated).is_ok());

        if !char_is_in_one_set {
            errors.push(Error {
                kind: ErrorKind::NotInSet,
                value: format!("Char {} in position {} not in set", c, i),
            });
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

/*
    FUNCTION : string must validate ALL functions
    SET      : each char of string must validate ONE (or more) function
*/
pub enum ValidatingConditionField {
    FUNCTION(Vec<fn(&str, bool) -> Result<(), Error>>),
    SET(Vec<fn(&char, bool) -> Result<(), Error>>)
}

impl Specification for FieldSpecification{}