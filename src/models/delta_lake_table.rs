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
pub struct DeltaLakeTable {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metadataLocationHistory")]
    pub metadata_location_history: Vec<String>,
    #[serde(rename = "checkpointLocationHistory")]
    pub checkpoint_location_history: Vec<String>,
    #[serde(rename = "lastCheckpoint", skip_serializing_if = "Option::is_none")]
    pub last_checkpoint: Option<String>,
}

impl DeltaLakeTable {
    pub fn new(metadata_location_history: Vec<String>, checkpoint_location_history: Vec<String>) -> DeltaLakeTable {
        DeltaLakeTable {
            id: None,
            metadata_location_history,
            checkpoint_location_history,
            last_checkpoint: None,
        }
    }
}

