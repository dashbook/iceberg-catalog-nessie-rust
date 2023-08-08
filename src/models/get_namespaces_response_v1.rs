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
pub struct GetNamespacesResponseV1 {
    #[serde(rename = "namespaces")]
    pub namespaces: Vec<crate::models::NamespaceV1>,
}

impl GetNamespacesResponseV1 {
    pub fn new(namespaces: Vec<crate::models::NamespaceV1>) -> GetNamespacesResponseV1 {
        GetNamespacesResponseV1 {
            namespaces,
        }
    }
}

