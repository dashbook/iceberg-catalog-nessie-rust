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
pub struct UdfV1 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "sqlText")]
    pub sql_text: String,
    #[serde(rename = "dialect", skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
}

impl UdfV1 {
    pub fn new(sql_text: String) -> UdfV1 {
        UdfV1 {
            id: None,
            sql_text,
            dialect: None,
        }
    }
}


