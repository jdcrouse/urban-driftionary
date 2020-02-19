use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DefinitionsResult {
    pub term: String,
    pub definitions: Vec<DefinitionDetail>,
}

#[derive(Serialize, Deserialize)]
pub struct DefinitionDetail {
    pub definition: String,
    pub example_sentence: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AddDefinition {
    pub term: String,
    pub definition: String,
    pub example_sentence: String,
    pub tags: Vec<String>,
}

pub fn add_definition(definition: AddDefinition) -> Result<String, &'static str> {
    Ok(definition.term) // TODO
}

pub fn get_definition(term: String) -> Option<DefinitionsResult> {
    Some(DefinitionsResult {
        term: String::from("Drift"),
        definitions: vec![DefinitionDetail {
            definition: String::from("A wonderful place to work"),
            example_sentence: String::from("I sure do enjoy working at Drift"),
            tags: vec![
                String::from("company"),
                String::from("marketing"),
                String::from("sales"),
            ],
        }],
    })
}
