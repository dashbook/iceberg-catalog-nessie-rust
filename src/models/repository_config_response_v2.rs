/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RepositoryConfigResponseV2 : The existing configuration objects for the requested types will be returned. Non-existing config objects will not be returned.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepositoryConfigResponseV2 {
    /// The existing configuration objects for the requested types will be returned. Non-existing config objects will not be returned.
    #[serde(rename = "configs", skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<crate::models::RepositoryConfigV2>>,
}

impl RepositoryConfigResponseV2 {
    /// The existing configuration objects for the requested types will be returned. Non-existing config objects will not be returned.
    pub fn new() -> RepositoryConfigResponseV2 {
        RepositoryConfigResponseV2 {
            configs: None,
        }
    }
}

