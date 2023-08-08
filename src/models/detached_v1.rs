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
pub struct DetachedV1 {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::ReferenceMetadataV1>>,
}

impl DetachedV1 {
    pub fn new(hash: String) -> DetachedV1 {
        DetachedV1 {
            hash,
            metadata: None,
        }
    }
}

