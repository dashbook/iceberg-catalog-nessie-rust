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
pub struct DiffResponse {
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "diffs", skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<crate::models::DiffEntry>>,
    #[serde(rename = "effectiveFromReference", skip_serializing_if = "Option::is_none")]
    pub effective_from_reference: Option<Box<crate::models::Reference>>,
    #[serde(rename = "effectiveToReference", skip_serializing_if = "Option::is_none")]
    pub effective_to_reference: Option<Box<crate::models::Reference>>,
}

impl DiffResponse {
    pub fn new() -> DiffResponse {
        DiffResponse {
            has_more: None,
            token: None,
            diffs: None,
            effective_from_reference: None,
            effective_to_reference: None,
        }
    }
}


