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
pub struct CommitMeta {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "committer", skip_serializing_if = "Option::is_none")]
    pub committer: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "authors")]
    pub authors: Vec<String>,
    #[serde(rename = "signedOffBy", skip_serializing_if = "Option::is_none")]
    pub signed_off_by: Option<String>,
    #[serde(rename = "allSignedOffBy")]
    pub all_signed_off_by: Vec<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "commitTime", skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<String>,
    #[serde(rename = "authorTime", skip_serializing_if = "Option::is_none")]
    pub author_time: Option<String>,
    #[serde(rename = "properties")]
    pub properties: ::std::collections::HashMap<String, String>,
    #[serde(rename = "allProperties")]
    pub all_properties: ::std::collections::HashMap<String, Vec<String>>,
    #[serde(rename = "parentCommitHashes")]
    pub parent_commit_hashes: Vec<String>,
}

impl CommitMeta {
    pub fn new(authors: Vec<String>, all_signed_off_by: Vec<String>, message: String, properties: ::std::collections::HashMap<String, String>, all_properties: ::std::collections::HashMap<String, Vec<String>>, parent_commit_hashes: Vec<String>) -> CommitMeta {
        CommitMeta {
            hash: None,
            committer: None,
            author: None,
            authors,
            signed_off_by: None,
            all_signed_off_by,
            message,
            commit_time: None,
            author_time: None,
            properties,
            all_properties,
            parent_commit_hashes,
        }
    }
}


