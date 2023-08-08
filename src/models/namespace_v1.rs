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
pub struct NamespaceV1 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "elements")]
    pub elements: Vec<String>,
    #[serde(rename = "properties")]
    pub properties: ::std::collections::HashMap<String, String>,
}

impl NamespaceV1 {
    pub fn new(elements: Vec<String>, properties: ::std::collections::HashMap<String, String>) -> NamespaceV1 {
        NamespaceV1 {
            id: None,
            elements,
            properties,
        }
    }
}


