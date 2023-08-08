# NessieConfigurationV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_branch** | Option<**String**> |  | [optional]
**min_supported_api_version** | Option<**i32**> |  | [optional]
**max_supported_api_version** | Option<**i32**> |  | [optional]
**actual_api_version** | Option<**i32**> |  | [optional]
**spec_version** | Option<**String**> | Semver version representing the behavior of the Nessie server.  Additional functionality might be added to Nessie servers within a \"spec major version\" in a non-breaking way. Clients are encouraged to check the spec version when using such added functionality. | [optional]
**no_ancestor_hash** | Option<**String**> |  | [optional]
**repository_creation_timestamp** | Option<**String**> |  | [optional]
**oldest_possible_commit_timestamp** | Option<**String**> |  | [optional]
**additional_properties** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


