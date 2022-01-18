use std::{collections::HashMap};

use crate::specifications::{SpecificationsStore, ValidatingConditionField, FieldSpecification};

use super::field_validation_functions::*;
use super::field_validation_sets::*;

lazy_static! {
    static ref STORE: FieldSpecificationsStore = FieldSpecificationsStore::new();
}

pub struct FieldSpecificationsStore{
    map: HashMap<&'static str, FieldSpecification>
}

impl FieldSpecificationsStore {
    fn new() -> FieldSpecificationsStore{
        let mut store = FieldSpecificationsStore {
            map: HashMap::new()
        };

        Self::_fill(&mut store.map);

        store
    }
}

impl SpecificationsStore<FieldSpecification> for FieldSpecificationsStore{

    fn get(id: &str) -> Option<&FieldSpecification> {
        STORE.map.get(id)
    }

    fn _fill(hash: &mut HashMap<&'static str, FieldSpecification>) {
        include!(concat!(env!("OUT_DIR"), "/identifier_loader_inserts.rs"))
    }
}