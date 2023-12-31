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
pub struct CommitMetaV1 {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "committer", skip_serializing_if = "Option::is_none")]
    pub committer: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "signedOffBy", skip_serializing_if = "Option::is_none")]
    pub signed_off_by: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "commitTime", skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<String>,
    #[serde(rename = "authorTime", skip_serializing_if = "Option::is_none")]
    pub author_time: Option<String>,
    #[serde(rename = "properties")]
    pub properties: ::std::collections::HashMap<String, String>,
}

impl CommitMetaV1 {
    pub fn new(message: String, properties: ::std::collections::HashMap<String, String>) -> CommitMetaV1 {
        CommitMetaV1 {
            hash: None,
            committer: None,
            author: None,
            signed_off_by: None,
            message,
            commit_time: None,
            author_time: None,
            properties,
        }
    }
}


