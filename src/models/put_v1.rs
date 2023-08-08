/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PutV1 : Used to add new content or to update existing content.  A new content object is created by populating the `value` field, the content-id in the content object must not be present (null).  A content object is updated by populating the `value` containing the correct content-id.  If the key for a content shall change (aka a rename), then use a `Delete` operation using the current (old) key and a `Put` operation using the new key with the `value` having the correct content-id. Both operations must happen in the same commit.  A content object can be replaced (think: `DROP TABLE xyz` + `CREATE TABLE xyz`) with a `Delete` operation and a `Put` operation for a content using a `value`representing a new content object, so without a content-id, in the same commit.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutV1 {
    #[serde(rename = "key")]
    pub key: Box<crate::models::ContentKeyV1>,
    #[serde(rename = "content")]
    pub content: Box<crate::models::ContentV1>,
    #[serde(rename = "expectedContent", skip_serializing_if = "Option::is_none")]
    pub expected_content: Option<Box<crate::models::PutV1ExpectedContent>>,
}

impl PutV1 {
    /// Used to add new content or to update existing content.  A new content object is created by populating the `value` field, the content-id in the content object must not be present (null).  A content object is updated by populating the `value` containing the correct content-id.  If the key for a content shall change (aka a rename), then use a `Delete` operation using the current (old) key and a `Put` operation using the new key with the `value` having the correct content-id. Both operations must happen in the same commit.  A content object can be replaced (think: `DROP TABLE xyz` + `CREATE TABLE xyz`) with a `Delete` operation and a `Put` operation for a content using a `value`representing a new content object, so without a content-id, in the same commit.
    pub fn new(key: crate::models::ContentKeyV1, content: crate::models::ContentV1) -> PutV1 {
        PutV1 {
            key: Box::new(key),
            content: Box::new(content),
            expected_content: None,
        }
    }
}
