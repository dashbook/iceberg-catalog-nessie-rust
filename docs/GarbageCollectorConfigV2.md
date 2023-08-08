# GarbageCollectorConfigV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_cutoff_policy** | Option<**String**> | The default cutoff policy. Policies can be one of: - number of commits as an integer value - a duration (see java.time.Duration) - an ISO instant - 'NONE', means everything's considered as live | [optional]
**per_ref_cutoff_policies** | Option<[**Vec<crate::models::ReferenceCutoffPolicyV2>**](ReferenceCutoffPolicy_V2.md)> |  | [optional]
**new_files_grace_period** | Option<**String**> | Files that have been created after 'gc-start-time - new-files-grace-period' are not being deleted. | [optional]
**expected_file_count_per_content** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


