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
pub struct SingleReferenceResponse {
    #[serde(rename = "reference")]
    pub reference: Box<crate::models::Reference>,
}

impl SingleReferenceResponse {
    pub fn new(reference: crate::models::Reference) -> SingleReferenceResponse {
        SingleReferenceResponse {
            reference: Box::new(reference),
        }
    }
}
