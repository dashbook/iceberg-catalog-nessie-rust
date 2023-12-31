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
pub struct ContentWithKeyV1 {
    #[serde(rename = "key")]
    pub key: Box<crate::models::ContentKeyV1>,
    #[serde(rename = "content")]
    pub content: Box<crate::models::ContentV1>,
}

impl ContentWithKeyV1 {
    pub fn new(
        key: crate::models::ContentKeyV1,
        content: crate::models::ContentV1,
    ) -> ContentWithKeyV1 {
        ContentWithKeyV1 {
            key: Box::new(key),
            content: Box::new(content),
        }
    }
}
