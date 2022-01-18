use std::collections::HashMap;

mod document;
pub use document::*;

mod field;
pub use field::*;

mod error;
pub use error::{Error, ErrorKind};

pub trait Specification{}

pub trait SpecificationsStore<S: Specification> {
    /*
        filled in build.rs
    */
    fn _fill(hash: &mut HashMap<&'static str, S>);

    fn get(id: &str) -> Option<&S>;
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn store_document(){
        let ds = DocumentSpecificationsStore::get("10").expect("Not found");
    
        assert_eq!(ds.id, "10");
        assert_eq!(ds.type_document, "Justificatif d’emploi");
        assert_eq!(ds.date_emission, true);
        assert_eq!(ds.description, "Contrat de travail");
        assert!(ds.validate(&vec!["61", "62", "5A", "57", "50",   "0A", "05"]).is_ok());
    }
    
    #[test]
    fn store_identifier(){
        let fs = FieldSpecificationsStore::get("10").expect("Not found");
    
        assert_eq!(fs.id, "10");
        assert_eq!(fs.nom, "Ligne 1 de la norme adresse postale du bénéficiaire de la prestation");
        assert_eq!(fs.taille_min, 0);
        assert_eq!(fs.taille_max, 38);
        assert_eq!(fs.type_identifiant, "Alphanumérique");
        assert!(fs.validate("MR/JEAN/DE LA FONTAINE", false).is_ok());
   }
}
