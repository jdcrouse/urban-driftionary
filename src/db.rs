use elasticsearch::{
    http::transport::{SingleNodeConnectionPool, Transport},
    Elasticsearch, Error, SearchParts,
};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize)]
pub struct ToBeDefined {
    pub term: String,
}

const AWS_ES_ENDPOINT: &str = "https://search-es-driftionary-kq77fbn6hvsqt3psu3htuga7gi.us-east-1.es.amazonaws.com/es-driftionary";
const SEARCH_PATH: &str = "/_search";

pub fn add_definition(definition: AddDefinition) -> Result<String, &'static str> {
    Ok(definition.term) // TODO: implement
}

pub async fn get_definition(term: String) -> Option<DefinitionsResult> {
    match search_elastic(term).await {
        Ok(defn) => Some(defn),
        Err(_e) => None,
    }
}

pub fn request_to_be_defined(tbd: ToBeDefined) -> Result<String, &'static str> {
    Ok(tbd.term) // TODO: implement
}

async fn search_elastic_reqwest(_term: String) -> Result<DefinitionsResult, reqwest::Error> {
    let mut body = HashMap::new();
    let mut query: HashMap<String, HashMap<_, _>> = HashMap::new();
    let match_all: HashMap<String, String> = HashMap::new();
    query.insert("match_all".to_owned(), match_all);
    body.insert("query", query);

    let client = reqwest::Client::new();
    let endpoint = format!("{}{}", &AWS_ES_ENDPOINT, &SEARCH_PATH);
    let res = client.post(&endpoint).json(&body).send().await?;
    println!("{}", res.status());

    let content = res.text().await?;
    println!("{}", content);

    Ok(dummy_get_response()) // TODO: remove
}

async fn search_elastic(term: String) -> Result<DefinitionsResult, elasticsearch::Error> {
    let transport = Transport::single_node(AWS_ES_ENDPOINT)?;
    let client = Elasticsearch::new(transport);
    let search_response = client
        .search(SearchParts::None)
        .body(json!({
          "query": {
              "match_phrase": {
                  "term": term
                }
          }
        }))
        .allow_no_indices(true)
        .send()
        .await?;

    // read the response body. Consumes search_response
    let response_body = search_response.read_body::<Value>().await?;
    println!("{}", response_body);

    let hits = response_body["hits"].as_object().unwrap();
    let hits = hits["hits"].as_array().unwrap();

    println!("{} hits", hits.len());
    Ok(dummy_get_response()) // TODO: remove
}

fn dummy_get_response() -> DefinitionsResult {
    // TODO: remove
    DefinitionsResult {
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
    }
}
