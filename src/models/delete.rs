/*
 * Nessie API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.65.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Delete : Used to delete an existing content key.  If the key for a content shall change (aka a rename), then use a `Delete` operation using the current (old) key and a `Put` operation using the new key with the current `Content` in the the `value` field. See `Put` operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "key")]
    pub key: Box<crate::models::ContentKey>,
}

impl Delete {
    /// Used to delete an existing content key.  If the key for a content shall change (aka a rename), then use a `Delete` operation using the current (old) key and a `Put` operation using the new key with the current `Content` in the the `value` field. See `Put` operation.
    pub fn new(key: crate::models::ContentKey) -> Delete {
        Delete {
            key: Box::new(key),
        }
    }
}


