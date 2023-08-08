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
pub struct DiffResponseV1 {
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "diffs", skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<crate::models::DiffEntryV1>>,
}

impl DiffResponseV1 {
    pub fn new() -> DiffResponseV1 {
        DiffResponseV1 {
            has_more: None,
            token: None,
            diffs: None,
        }
    }
}


