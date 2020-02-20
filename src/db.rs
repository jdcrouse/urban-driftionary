use elasticsearch::{http::transport::Transport, Elasticsearch, SearchParts};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct Hit {
    _id: String,
    _source: DefinitionsResult,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DefinitionsResult {
    pub term: String,
    pub definitions: Vec<DefinitionDetail>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DefinitionDetail {
    pub definition: String,
    pub example: String,
    pub tags: Vec<Tag>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Tag {
    pub tag_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddDefinition {
    pub term: String,
    pub definition: String,
    pub example_sentence: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ToBeDefined {
    pub term: String,
}

const AWS_ES_ENDPOINT: &str = "https://search-es-driftionary-kq77fbn6hvsqt3psu3htuga7gi.us-east-1.es.amazonaws.com/driftionary";

pub fn add_definition(definition: AddDefinition) -> Result<String, &'static str> {
    Ok(definition.term) // TODO: implement
}

pub fn request_to_be_defined(tbd: ToBeDefined) -> Result<String, &'static str> {
    Ok(tbd.term) // TODO: implement
}

pub async fn get_definition(term: String) -> Option<DefinitionsResult> {
    let hits = search_elastic(term).await;
    if hits.is_empty() {
        None
    } else {
        let first_result = hits.get(0).unwrap();
        Some(first_result._source.clone())
    }
}

async fn search_elastic(term: String) -> Vec<Hit> {
    let transport = Transport::single_node(AWS_ES_ENDPOINT).unwrap();
    let client = Elasticsearch::new(transport);
    let search_response = client
        .search(SearchParts::None)
        .body(json!(
        {
            "query": {
                "term": {
                    "term.raw": term
                  }
            }
          }))
        .allow_no_indices(true)
        .send()
        .await
        .unwrap();
    let response_body = search_response.read_body::<Value>().await.unwrap();
    let hits = response_body["hits"].as_object().unwrap();
    let hits: Vec<Hit> = serde_json::from_value(hits["hits"].to_owned()).unwrap();
    hits
}
