/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReferenceV1 {
    #[serde(rename = "BRANCH")]
    BranchV1 {
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
        metadata: Option<Box<crate::models::ReferenceMetadataV1>>,
        #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
        hash: Option<String>,
    },
    #[serde(rename = "DETACHED")]
    DetachedV1 {
        #[serde(rename = "hash")]
        hash: String,
        #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
        metadata: Option<Box<crate::models::ReferenceMetadataV1>>,
    },
    #[serde(rename = "TAG")]
    TagV1 {
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
        metadata: Option<Box<crate::models::ReferenceMetadataV1>>,
        #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
        hash: Option<String>,
    },
}
