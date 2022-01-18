use std::{fs, path::PathBuf};

use serde_json::{Value};

pub struct Builder{
    s6: Value,
    s7: Value,
    s8: Value
}

impl Builder{
    pub fn new(json_path: &str) -> Builder{        
        let base_path = PathBuf::from(json_path);

        Builder { 
            s6: Self::read_to_json(&base_path, "s6.json"),
            s7: Self::read_to_json(&base_path, "s7.json"),
            s8: Self::read_to_json(&base_path, "s8.json")
        }
    }

    fn read_to_json(base_path: &PathBuf, file_name: &str) -> Value{
        serde_json::from_str(fs::read_to_string(base_path.join(file_name)).unwrap().as_str()).unwrap()
    }

    pub fn document_inserts(&self) -> String{
        self.s6.as_object().unwrap().iter().fold(String::from("{"), |acc, (key, value)| format!(r#"{acc}
                hash.insert(
                    "{id}",
                    DocumentSpecification {{
                        id: "{id}",
                        type_document: "{type_doc}",
                        date_emission: {date_bool},
                        description: "{description}",
                        validate: {validate},
                    }}
                );
            "#,
            acc=acc,
            id=key,
            type_doc=value["type"].as_str().unwrap(),
            date_bool=value["date_emission"].as_bool().unwrap(),
            description=value["description"].as_str().unwrap(),
            validate=document_condition_gen(&self.s8.as_object().unwrap()[key])
        )) + "}"
    }

    pub fn identifiers_inserts(&self) -> String{
        self.s7.as_object().unwrap().iter().fold(String::from("{"), |acc, (key, value)| format!(r#"{acc}
                hash.insert(
                    "{id}", 
                    FieldSpecification {{
                        id: "{id}",
                        nom: "{nom}",
                        taille_min: {taille_min},
                        taille_max: {taille_max},
                        type_identifiant: "{type_identifiant}",
                        description: "{description}",
                        validate: ValidatingConditionField::{validate_type}(vec![{validate_conditions}])
                    }}
                );
            "#,
            acc=acc,
            id=key,
            nom=value["nom"].as_str().unwrap(),
            taille_min=value["taille_min"].as_i64().unwrap(),
            taille_max=value["taille_max"].as_i64().unwrap(),
            type_identifiant=value["type"].as_str().unwrap(),
            description=value["description"].as_str().unwrap(),
            validate_type=validate_type(value["condition_type"].as_str().unwrap()),
            validate_conditions=validate_conditions(value["condition_type"].as_str().unwrap(), value["condition"].as_str().unwrap()),
        )) + "}"
    }
}

fn validate_conditions(condition_type: &str, conditions: &str) -> String{
    match condition_type {
        "F" => function_list(conditions, "validate_f_"),
        "S" => function_list(conditions, "validate_s_"),
        _   => panic!("Validating condition must be F or S")
    }
}

fn function_list(conditions:&str, prefix: &str) -> String {
    conditions
        .split('+')
        .map(|name| format!("{}{}", prefix, name))
        .collect::<Vec<String>>()
        .join(",")
}

fn validate_type(condition_type: &str) -> &str{
    match condition_type {
        "F" => "FUNCTION",
        "S" => "SET",
        _   => panic!("Validating condition must be F or S")
    }
}

fn document_condition_gen(condition: &Value) -> String {
    /*
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
        
    */
    let condition = condition.as_object().unwrap();

    format!(r#"
        ValidatingConditionDocument::Set(SetsCondition::new(
            {},
            {},
            {}
        ))"#,
        &document_key_to_vec(&condition["O"]),
        &document_key_to_vec(&condition["F"]),
        &document_build_condition(&condition["C"])
    )

}

fn document_key_to_vec(data: &Value) -> String {
    /*
        builds vec![ "C11".to_string(), "C12".to_string()]
    */
    let mut res = data.as_array().unwrap().iter()
                        .map(|v| v.as_str().unwrap())
                        .fold(String::new(), |acc, v| format!(r#"{}"{}".to_string(),"#, acc, v));
    res.pop();

    format!("vec![{}]", res)
}

fn document_build_condition(data: &Value) -> String{
    /*
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
    */
    let mut res = data.as_array().unwrap().iter()
        .map(|v| v.as_array().unwrap())
        .fold(String::new(), |acc, v| 
            format!(r#"{}
                AorB::new(
                    {},
                    {}
                ),"#,
                acc,
                document_key_to_vec(&v[0]),
                document_key_to_vec(&v[1])
            )
        );
    
    res.pop();

    format!("vec![{}]", res)

}