use std::{collections::HashSet, iter::FromIterator};

use crate::specifications::{Error, Specification, ErrorKind};

pub struct DocumentSpecification {
    pub id: &'static str,
    pub type_document: &'static str,
    pub date_emission: bool,
    pub description: &'static str,
    pub validate: ValidatingConditionDocument
}

impl DocumentSpecification {
    pub fn validate(&self, value: &[&str]) -> Result<(), Error> {
        match &self.validate {
            ValidatingConditionDocument::Function(f) => f(value),
            ValidatingConditionDocument::Set(s) => validate_sets(s, value),
        }
    }
}

fn validate_sets(base: &SetsCondition, document_fields: &[&str]) -> Result<(), Error>{
    let (mandatory, optional, conditional) = base.clone_and_explode();

    let mut f1: HashSet<String> = document_fields.iter()
        .map(|&s| s.to_string())
        .filter(|s| !optional.contains(s))
        .collect();

    for c in &conditional{
        if f1.is_superset(&c.a) && f1.is_disjoint(&c.b){
            //Contains A and no elements from B
            for s in &c.a {
                f1.remove(s);
            }
        }else if f1.is_superset(&c.b) && f1.is_disjoint(&c.a){
            //Contains B and no elements from A
            for s in &c.b {
                f1.remove(s);
            }
        }
    }


    let f2 = f1.symmetric_difference(&mandatory);

    if f2.count() != 0 {
        return Err(Error {
            kind: ErrorKind::DocumentFields,
            value: String::from("Document fields combinaison is not valid"),
        })
    }

    Ok(())
}

impl Specification for DocumentSpecification{}

pub enum ValidatingConditionDocument {
    #[allow(unused)] //For edge case document validation
    Function(fn(&[&str]) -> Result<(), Error>),
    Set(SetsCondition),
}

#[derive(Clone)]
pub struct SetsCondition{
    mandatory: HashSet<String>,
    optional: HashSet<String>,
    conditional: Vec<AorB>
}

impl SetsCondition{
    pub fn clone_and_explode(&self) -> (HashSet<String>, HashSet<String>, Vec<AorB>){
        let clone = self.clone();

        (clone.mandatory, clone.optional, clone.conditional)
    }

    pub fn new(mut mandatory: Vec<String>, mut optional: Vec<String>, conditional: Vec<AorB>) -> SetsCondition{
        SetsCondition {
            mandatory: HashSet::from_iter(mandatory.drain(..)),
            optional: HashSet::from_iter(optional.drain(..)),
            conditional,
        }
    }
}

#[derive(Clone)]
pub struct AorB {
    pub a: HashSet<String>,
    pub b: HashSet<String>
}

impl AorB{
    pub fn new(mut a: Vec<String>, mut b: Vec<String>) -> AorB{
        AorB{
            a: HashSet::from_iter(a.drain(..)),
            b: HashSet::from_iter(b.drain(..))
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        let sets = get_sets();

        let doc = vec!["MA", "MB", "OA", "C21", "C22"];

        let r = validate_sets(&sets, doc.as_slice());

        assert!(r.is_ok())

    }

    fn get_sets() -> SetsCondition {
        SetsCondition::new(
            vec![
                "MA".to_string(),
                "MB".to_string()
            ],
            vec![
                "OA".to_string(),
                "OB".to_string()
            ],
            vec![
                AorB::new(
                    vec![
                        "C11".to_string(),
                        "C12".to_string()
                    ],
                    vec![
                        "C21".to_string(),
                        "C22".to_string()
                    ]
                )
            ]
        )
    }
}