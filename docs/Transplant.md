# Transplant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<**String**> | Commit message for this transplant request. | [optional]
**hashes_to_transplant** | **Vec<String>** | Lists the hashes of commits that should be transplanted into the target branch. | 
**from_ref_name** | **String** | The name of the reference that contains the 'source' commits for the requested merge or transplant operation.  | 
**key_merge_modes** | Option<[**Vec<crate::models::MergeKeyBehavior>**](MergeKeyBehavior.md)> | Specific merge behaviour requests by content key.  The default is set by the `defaultKeyMergeMode` parameter.  | [optional]
**default_key_merge_mode** | Option<[**crate::models::MergeBehavior**](MergeBehavior.md)> |  | [optional]
**dry_run** | Option<**bool**> | When set to 'true' instructs the server to validate the request but to avoid committing any changes.  | [optional]
**fetch_additional_info** | Option<**bool**> | Whether to provide optional response data.  | [optional]
**return_conflict_as_result** | Option<**bool**> | When set to 'true' instructs the server to produce normal (non-error) responses in case a conflict is detected and report conflict details in the response payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


