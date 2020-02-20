use elasticsearch::{http::transport::Transport, Elasticsearch, Error, Search, SearchParts};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

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

pub struct ElasticClient {
    access_key: String,
    secret_key: String,
    client: Elasticsearch,
}

impl ElasticClient {
    pub fn new() -> ElasticClient {
        let aws_access_key = env::var("AWS_ACCESS_KEY").unwrap();
        let aws_secret_key = env::var("AWS_SECRET_KEY").unwrap();
        let transport = Transport::single_node("https://search-es-driftionary-kq77fbn6hvsqt3psu3htuga7gi.us-east-1.es.amazonaws.com/es-driftionary").unwrap();
        let client = Elasticsearch::new(transport);
        ElasticClient {
            access_key: aws_access_key,
            secret_key: aws_secret_key,
            client: client,
        }
    }

    pub fn search<'a, 'b>(&'a self, parts: SearchParts<'b>) -> Search<'a, 'b, ()> {
        self.client.search(parts)
    }
}

pub fn add_definition(definition: AddDefinition) -> Result<String, &'static str> {
    Ok(definition.term) // TODO: implement
}

fn dummy_get_response() -> DefinitionsResult {
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

pub async fn get_definition(client: ElasticClient, term: String) -> Option<DefinitionsResult> {
    match search_elastic(client, term).await {
        Ok(defn) => Some(defn),
        Err(e) => None,
    }
}

async fn search_elastic(client: ElasticClient, term: String) -> Result<DefinitionsResult, Error> {
    // make a search API call
    let search_response = client
        .search(SearchParts::None)
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .allow_no_indices(true)
        .send()
        .await?;
    // get the HTTP response status code
    let status_code = search_response.status_code();
    // read the response body. Consumes search_response
    let response_body = search_response.read_body::<Value>().await?;

    // read fields from the response body
    let took = response_body["took"].as_i64().unwrap();

    Ok(dummy_get_response()) // TODO: remove
}

pub fn request_to_be_defined(tbd: ToBeDefined) -> Result<String, &'static str> {
    Ok(tbd.term) // TODO: implement
}
