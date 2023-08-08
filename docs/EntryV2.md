# EntryV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Declares the type of a Nessie content object, which is currently one of ICEBERG_TABLE, DELTA_LAKE_TABLE, ICEBERG_VIEW, NAMESPACE or UDF, which are the discriminator mapping values of the 'Content' type. | [optional]
**name** | [**crate::models::ContentKeyV2**](ContentKey_V2.md) |  | 
**content_id** | Option<**String**> |  | [optional]
**content** | Option<[**crate::models::ContentV2**](Content_V2.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


