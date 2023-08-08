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
pub struct EntryV2 {
    /// Declares the type of a Nessie content object, which is currently one of ICEBERG_TABLE, DELTA_LAKE_TABLE, ICEBERG_VIEW, NAMESPACE or UDF, which are the discriminator mapping values of the 'Content' type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "name")]
    pub name: Box<crate::models::ContentKeyV2>,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<crate::models::ContentV2>>,
}

impl EntryV2 {
    pub fn new(name: crate::models::ContentKeyV2) -> EntryV2 {
        EntryV2 {
            r#type: None,
            name: Box::new(name),
            content_id: None,
            content: None,
        }
    }
}


