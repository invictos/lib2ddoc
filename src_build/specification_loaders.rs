use std::fs;

use super::specification_builders;

pub struct Loader{
    builder: specification_builders::Builder
}

impl Loader{
    pub fn new(json_path: &str) -> Loader {
        Loader {
            builder: specification_builders::Builder::new(json_path)
        }
    }

    pub fn all(&self, document_inserts_path: &str, identifier_inserts_path: &str) -> Result<(), &'static str>{
        self.document_inserts(document_inserts_path)?;
        self.identifier_inserts(identifier_inserts_path)?;
        Ok(())
    }

    fn document_inserts(&self, path: &str) -> Result<(), &'static str>{
        let contents = self.builder.document_inserts();
        fs::write(path, contents).unwrap();
        Ok(())
    }
    
    fn identifier_inserts(&self, path: &str) -> Result<(), &'static str>{
        let contents = self.builder.identifiers_inserts();
        fs::write(path, contents).unwrap();
        Ok(())
    }
}

