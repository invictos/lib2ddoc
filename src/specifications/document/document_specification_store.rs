use std::collections::HashMap;

use crate::specifications::{SpecificationsStore, ValidatingConditionDocument, SetsCondition, AorB};

use super::DocumentSpecification;

lazy_static! {
    static ref STORE: DocumentSpecificationsStore = DocumentSpecificationsStore::new();
}

pub struct DocumentSpecificationsStore{
    map: HashMap<&'static str, DocumentSpecification>
}

impl DocumentSpecificationsStore {
    fn new() -> DocumentSpecificationsStore{
        let mut store = DocumentSpecificationsStore {
            map: HashMap::new()
        };

        Self::_fill(&mut store.map);

        store
    }
}

impl SpecificationsStore<DocumentSpecification> for DocumentSpecificationsStore{
    fn get(id: &str) -> Option<&DocumentSpecification> {
        STORE.map.get(id)
    }

    fn _fill(hash: &mut HashMap<&'static str, DocumentSpecification>) {
        include!(concat!(env!("OUT_DIR"), "/document_loader_inserts.rs"));
    }
}