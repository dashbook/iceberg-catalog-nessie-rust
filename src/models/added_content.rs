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
pub struct AddedContent {
    #[serde(rename = "key")]
    pub key: Box<crate::models::ContentKey>,
}

impl AddedContent {
    pub fn new(key: crate::models::ContentKey) -> AddedContent {
        AddedContent {
            key: Box::new(key),
        }
    }
}


