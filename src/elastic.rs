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
