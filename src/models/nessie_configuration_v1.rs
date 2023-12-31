/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NessieConfigurationV1 : Configuration object to tell a client how a server is configured.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NessieConfigurationV1 {
    #[serde(rename = "defaultBranch", skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(rename = "maxSupportedApiVersion", skip_serializing_if = "Option::is_none")]
    pub max_supported_api_version: Option<i32>,
}

impl NessieConfigurationV1 {
    /// Configuration object to tell a client how a server is configured.
    pub fn new() -> NessieConfigurationV1 {
        NessieConfigurationV1 {
            default_branch: None,
            max_supported_api_version: None,
        }
    }
}


