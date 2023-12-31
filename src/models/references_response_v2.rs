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
pub struct ReferencesResponseV2 {
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "references")]
    pub references: Vec<crate::models::ReferenceV2>,
}

impl ReferencesResponseV2 {
    pub fn new(references: Vec<crate::models::ReferenceV2>) -> ReferencesResponseV2 {
        ReferencesResponseV2 {
            has_more: None,
            token: None,
            references,
        }
    }
}


