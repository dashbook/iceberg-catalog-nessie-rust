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
pub struct ContentKeyDetailsV2 {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Box<crate::models::ContentKeyV2>>,
    #[serde(rename = "mergeBehavior", skip_serializing_if = "Option::is_none")]
    pub merge_behavior: Option<crate::models::MergeBehaviorV2>,
    #[serde(rename = "conflict", skip_serializing_if = "Option::is_none")]
    pub conflict: Option<serde_json::Value>,
}

impl ContentKeyDetailsV2 {
    pub fn new() -> ContentKeyDetailsV2 {
        ContentKeyDetailsV2 {
            key: None,
            merge_behavior: None,
            conflict: None,
        }
    }
}


