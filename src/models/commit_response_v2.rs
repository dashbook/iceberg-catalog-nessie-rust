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
pub struct CommitResponseV2 {
    #[serde(rename = "targetBranch")]
    pub target_branch: Box<crate::models::BranchV2>,
    #[serde(rename = "addedContents", skip_serializing_if = "Option::is_none")]
    pub added_contents: Option<Vec<crate::models::AddedContentV2>>,
}

impl CommitResponseV2 {
    pub fn new(target_branch: crate::models::BranchV2) -> CommitResponseV2 {
        CommitResponseV2 {
            target_branch: Box::new(target_branch),
            added_contents: None,
        }
    }
}


