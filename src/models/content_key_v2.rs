/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentKeyV2 {
    #[serde(rename = "elements")]
    pub elements: Vec<String>,
}

impl ContentKeyV2 {
    pub fn new(elements: Vec<String>) -> ContentKeyV2 {
        ContentKeyV2 {
            elements,
        }
    }
}

