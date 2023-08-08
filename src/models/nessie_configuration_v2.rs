/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NessieConfigurationV2 : Configuration object to tell a client how a server is configured.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NessieConfigurationV2 {
    #[serde(rename = "defaultBranch", skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(rename = "minSupportedApiVersion", skip_serializing_if = "Option::is_none")]
    pub min_supported_api_version: Option<i32>,
    #[serde(rename = "maxSupportedApiVersion", skip_serializing_if = "Option::is_none")]
    pub max_supported_api_version: Option<i32>,
    #[serde(rename = "actualApiVersion", skip_serializing_if = "Option::is_none")]
    pub actual_api_version: Option<i32>,
    /// Semver version representing the behavior of the Nessie server.  Additional functionality might be added to Nessie servers within a \"spec major version\" in a non-breaking way. Clients are encouraged to check the spec version when using such added functionality.
    #[serde(rename = "specVersion", skip_serializing_if = "Option::is_none")]
    pub spec_version: Option<String>,
    #[serde(rename = "noAncestorHash", skip_serializing_if = "Option::is_none")]
    pub no_ancestor_hash: Option<String>,
    #[serde(rename = "repositoryCreationTimestamp", skip_serializing_if = "Option::is_none")]
    pub repository_creation_timestamp: Option<String>,
    #[serde(rename = "oldestPossibleCommitTimestamp", skip_serializing_if = "Option::is_none")]
    pub oldest_possible_commit_timestamp: Option<String>,
    #[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<::std::collections::HashMap<String, String>>,
}

impl NessieConfigurationV2 {
    /// Configuration object to tell a client how a server is configured.
    pub fn new() -> NessieConfigurationV2 {
        NessieConfigurationV2 {
            default_branch: None,
            min_supported_api_version: None,
            max_supported_api_version: None,
            actual_api_version: None,
            spec_version: None,
            no_ancestor_hash: None,
            repository_creation_timestamp: None,
            oldest_possible_commit_timestamp: None,
            additional_properties: None,
        }
    }
}


