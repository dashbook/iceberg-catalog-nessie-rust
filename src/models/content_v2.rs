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
#[serde(tag = "r#type")]
pub enum ContentV2 {
    #[serde(rename="DELTA_LAKE_TABLE")]
    DeltaLakeTableV2 {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "metadataLocationHistory")]
        metadata_location_history: Vec<String>,
        #[serde(rename = "checkpointLocationHistory")]
        checkpoint_location_history: Vec<String>,
        #[serde(rename = "lastCheckpoint", skip_serializing_if = "Option::is_none")]
        last_checkpoint: Option<String>,
    },
    #[serde(rename="ICEBERG_TABLE")]
    IcebergTableV2 {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "metadataLocation")]
        metadata_location: String,
        #[serde(rename = "snapshotId", skip_serializing_if = "Option::is_none")]
        snapshot_id: Option<i64>,
        #[serde(rename = "schemaId", skip_serializing_if = "Option::is_none")]
        schema_id: Option<i32>,
        #[serde(rename = "specId", skip_serializing_if = "Option::is_none")]
        spec_id: Option<i32>,
        #[serde(rename = "sortOrderId", skip_serializing_if = "Option::is_none")]
        sort_order_id: Option<i32>,
    },
    #[serde(rename="ICEBERG_VIEW")]
    IcebergViewV2 {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "metadataLocation")]
        metadata_location: String,
        #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
        version_id: Option<i64>,
        #[serde(rename = "schemaId", skip_serializing_if = "Option::is_none")]
        schema_id: Option<i32>,
        #[serde(rename = "sqlText")]
        sql_text: String,
        #[serde(rename = "dialect", skip_serializing_if = "Option::is_none")]
        dialect: Option<String>,
    },
    #[serde(rename="NAMESPACE")]
    NamespaceV2 {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "elements")]
        elements: Vec<String>,
        #[serde(rename = "properties")]
        properties: ::std::collections::HashMap<String, String>,
    },
    #[serde(rename="UDF")]
    UdfV2 {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "sqlText")]
        sql_text: String,
        #[serde(rename = "dialect", skip_serializing_if = "Option::is_none")]
        dialect: Option<String>,
    },
}




