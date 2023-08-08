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
pub struct IcebergViewV1 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metadataLocation")]
    pub metadata_location: String,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i64>,
    #[serde(rename = "schemaId", skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<i32>,
    #[serde(rename = "sqlText")]
    pub sql_text: String,
    #[serde(rename = "dialect", skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl IcebergViewV1 {
    pub fn new(metadata_location: String, sql_text: String) -> IcebergViewV1 {
        IcebergViewV1 {
            id: None,
            metadata_location,
            version_id: None,
            schema_id: None,
            sql_text,
            dialect: None,
            metadata: None,
        }
    }
}


