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
pub struct MergeResponseV1 {
    #[serde(rename = "resultantTargetHash", skip_serializing_if = "Option::is_none")]
    pub resultant_target_hash: Option<String>,
    #[serde(rename = "commonAncestor", skip_serializing_if = "Option::is_none")]
    pub common_ancestor: Option<String>,
    #[serde(rename = "targetBranch", skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    #[serde(rename = "effectiveTargetHash", skip_serializing_if = "Option::is_none")]
    pub effective_target_hash: Option<String>,
    #[serde(rename = "expectedHash", skip_serializing_if = "Option::is_none")]
    pub expected_hash: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::ContentKeyDetailsV1>>,
}

impl MergeResponseV1 {
    pub fn new() -> MergeResponseV1 {
        MergeResponseV1 {
            resultant_target_hash: None,
            common_ancestor: None,
            target_branch: None,
            effective_target_hash: None,
            expected_hash: None,
            details: None,
        }
    }
}


