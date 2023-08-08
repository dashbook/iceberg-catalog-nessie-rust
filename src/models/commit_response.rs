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
pub struct CommitResponse {
    #[serde(rename = "targetBranch")]
    pub target_branch: Box<crate::models::Branch>,
    #[serde(rename = "addedContents", skip_serializing_if = "Option::is_none")]
    pub added_contents: Option<Vec<crate::models::AddedContent>>,
}

impl CommitResponse {
    pub fn new(target_branch: crate::models::Branch) -> CommitResponse {
        CommitResponse {
            target_branch: Box::new(target_branch),
            added_contents: None,
        }
    }
}

