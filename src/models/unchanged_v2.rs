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
pub struct UnchangedV2 {
    #[serde(rename = "key")]
    pub key: Box<crate::models::ContentKeyV2>,
}

impl UnchangedV2 {
    pub fn new(key: crate::models::ContentKeyV2) -> UnchangedV2 {
        UnchangedV2 {
            key: Box::new(key),
        }
    }
}

