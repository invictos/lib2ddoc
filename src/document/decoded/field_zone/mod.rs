use std::collections::HashMap;

pub mod decoder;

use crate::specifications::Error;

pub struct Field{
    pub id: String,
    pub value: String,
    pub is_truncated: bool,
    validity: Option<Result<(), Vec<Error>>>
}

impl Field {
    pub fn is_valid(&self) -> bool{
        if let Some(res) = &self.validity {
            return res.is_ok()
        }
        false
    }

    pub fn get_validity(&self) -> &Result<(), Vec<Error>> {
        if let Some(res) = &self.validity {
            return &res
        }
        panic!("Cannot get validation before validating");
    }

    pub fn set_validity_ok(&mut self) {
        self.validity = Some(Ok(()))
    }

    pub fn set_validity_err(&mut self, err: Vec<Error>) {
        self.validity = Some(Err(err))
    }

    pub fn get_validity_error_strings(&self) -> Option<Vec<String>> {
        self.get_validity().as_ref().err()
            .map(|f| 
                f.iter()
                .map(|e| e.to_string())
                .collect()
            )
    }
}

pub struct FieldZone{
    pub zone: HashMap<String, Field>,
}

impl FieldZone{
    pub fn get(&self,k:String)->Option<&Field>{
        self.zone.get(&k)
    }

    pub fn insert(&mut self,k:String,v:Field){
        self.zone.insert(k,v);
    }
}