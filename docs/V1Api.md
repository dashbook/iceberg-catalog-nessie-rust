# \V1Api

All URIs are relative to *http://localhost:19120/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_reference**](V1Api.md#assign_reference) | **PUT** /v1/trees/{referenceType}/{referenceName} | Set a named reference to a specific hash via a named-reference.
[**commit_multiple_operations**](V1Api.md#commit_multiple_operations) | **POST** /v1/trees/branch/{branchName}/commit | Commit multiple operations against the given branch expecting that branch to have the given hash as its latest commit. The hash in the successful response contains the hash of the commit that contains the operations of the invocation.
[**create_namespace**](V1Api.md#create_namespace) | **PUT** /v1/namespaces/namespace/{ref}/{name} | Creates a Namespace
[**create_reference**](V1Api.md#create_reference) | **POST** /v1/trees/tree | Create a new reference
[**delete_namespace**](V1Api.md#delete_namespace) | **DELETE** /v1/namespaces/namespace/{ref}/{name} | Deletes a Namespace
[**delete_reference**](V1Api.md#delete_reference) | **DELETE** /v1/trees/{referenceType}/{referenceName} | Delete a reference endpoint
[**get_all_references**](V1Api.md#get_all_references) | **GET** /v1/trees | Get all references
[**get_commit_log**](V1Api.md#get_commit_log) | **GET** /v1/trees/tree/{ref}/log | Get commit log for a reference
[**get_config**](V1Api.md#get_config) | **GET** /v1/config | List all configuration settings
[**get_content**](V1Api.md#get_content) | **GET** /v1/contents/{key} | Get object content associated with a key.
[**get_default_branch**](V1Api.md#get_default_branch) | **GET** /v1/trees/tree | Get default branch for commits and reads
[**get_diff**](V1Api.md#get_diff) | **GET** /v1/diffs/{fromRefWithHash}...{toRefWithHash} | Get a diff for two given references
[**get_entries**](V1Api.md#get_entries) | **GET** /v1/trees/tree/{ref}/entries | Fetch all entries for a given reference
[**get_multiple_contents**](V1Api.md#get_multiple_contents) | **POST** /v1/contents | Get multiple objects' content.
[**get_namespace**](V1Api.md#get_namespace) | **GET** /v1/namespaces/namespace/{ref}/{name} | Retrieves a Namespace
[**get_namespaces**](V1Api.md#get_namespaces) | **GET** /v1/namespaces/{ref} | 
[**get_ref_log**](V1Api.md#get_ref_log) | **GET** /v1/reflogs | Get reflog entries (DEPRECATED)
[**get_reference_by_name**](V1Api.md#get_reference_by_name) | **GET** /v1/trees/tree/{ref} | Fetch details of a reference
[**merge_ref_into_branch**](V1Api.md#merge_ref_into_branch) | **POST** /v1/trees/branch/{branchName}/merge | Merge commits from 'mergeRef' onto 'branchName'.
[**transplant_commits_into_branch**](V1Api.md#transplant_commits_into_branch) | **POST** /v1/trees/branch/{branchName}/transplant | Transplant commits from 'transplant' onto 'branchName'
[**update_properties**](V1Api.md#update_properties) | **POST** /v1/namespaces/namespace/{ref}/{name} | 



## assign_reference

> assign_reference(reference_name, reference_type, expected_hash, reference)
Set a named reference to a specific hash via a named-reference.

This operation takes the name of the named reference to reassign and the hash and the name of a named-reference via which the caller has access to that hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_name** | **String** | Reference name to reassign | [required] |
**reference_type** | [**ReferenceType**](.md) | Reference type to reassign | [required] |
**expected_hash** | **String** | Expected previous hash of reference | [required] |
**reference** | Option<[**Reference**](Reference.md)> | Reference hash to which 'referenceName' shall be assigned to. This must be either a 'Transaction', 'Branch' or 'Tag' via which the hash is visible to the caller. |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_multiple_operations

> crate::models::BranchV1 commit_multiple_operations(branch_name, expected_hash, operations)
Commit multiple operations against the given branch expecting that branch to have the given hash as its latest commit. The hash in the successful response contains the hash of the commit that contains the operations of the invocation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch_name** | **String** | Branch to change, defaults to default branch. | [required] |
**expected_hash** | Option<**String**> | Expected hash of branch. |  |
**operations** | Option<[**Operations**](Operations.md)> | Operations |  |

### Return type

[**crate::models::BranchV1**](Branch_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespace

> crate::models::NamespaceV1 create_namespace(name, r#ref, hash_on_ref, namespace)
Creates a Namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**Namespace**](.md) | the name of the namespace | [required] |
**r#ref** | **String** | name of ref to fetch | [required] |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |
**namespace** | Option<[**Namespace**](Namespace.md)> |  |  |

### Return type

[**crate::models::NamespaceV1**](Namespace_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reference

> crate::models::ReferenceV1 create_reference(source_ref_name, reference)
Create a new reference

The type of 'refObj', which can be either a 'Branch' or 'Tag', determines the type of the reference to be created.  'Reference.name' defines the the name of the reference to be created,'Reference.hash' is the hash of the created reference, the HEAD of the created reference. 'sourceRefName' is the name of the reference which contains 'Reference.hash', and must be present if 'Reference.hash' is present.  Specifying no 'Reference.hash' means that the new reference will be created \"at the beginning of time\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_ref_name** | Option<**String**> | Source named reference |  |
**reference** | Option<[**Reference**](Reference.md)> | Reference to create. |  |

### Return type

[**crate::models::ReferenceV1**](Reference_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespace

> delete_namespace(name, r#ref, hash_on_ref)
Deletes a Namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**Namespace**](.md) | the name of the namespace | [required] |
**r#ref** | **String** | name of ref to fetch | [required] |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_reference

> delete_reference(reference_name, reference_type, expected_hash)
Delete a reference endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_name** | **String** | Reference name to delete | [required] |
**reference_type** | [**ReferenceType**](.md) | Reference type to delete | [required] |
**expected_hash** | **String** | Expected hash of tag | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_references

> crate::models::ReferencesResponseV1 get_all_references(fetch, filter, max_records, page_token)
Get all references

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fetch** | Option<[**FetchOption**](.md)> | Specify how much information to be returned. Will fetch additional metadata for references if set to 'ALL'.  A returned Branch instance will have the following information:  - numCommitsAhead (number of commits ahead of the default branch)  - numCommitsBehind (number of commits behind the default branch)  - commitMetaOfHEAD (the commit metadata of the HEAD commit)  - commonAncestorHash (the hash of the common ancestor in relation to the default branch).  - numTotalCommits (the total number of commits in this reference).  A returned Tag instance will only contain the 'commitMetaOfHEAD' and 'numTotalCommits' fields.  Note that computing & fetching additional metadata might be computationally expensive on the server-side, so this flag should be used with care. |  |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md. Usable variables within the expression are:  - ref (Reference) describes the reference, with fields name (String), hash (String), metadata (ReferenceMetadata)  - metadata (ReferenceMetadata) shortcut to ref.metadata, never null, but possibly empty  - commit (CommitMeta) - shortcut to ref.metadata.commitMetaOfHEAD, never null, but possibly empty  - refType (String) - the reference type, either BRANCH or TAG  Note that the expression can only test attributes metadata and commit, if 'fetchOption' is set to 'ALL'. |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |

### Return type

[**crate::models::ReferencesResponseV1**](ReferencesResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_commit_log

> crate::models::LogResponseV1 get_commit_log(r#ref, end_hash, fetch, filter, max_records, page_token, start_hash)
Get commit log for a reference

Retrieve the commit log for a ref, potentially truncated by the backend.  Retrieves up to 'maxRecords' commit-log-entries starting at the HEAD of the given named reference (tag or branch) or the given hash. The backend may respect the given 'max' records hint, but return less or more entries. Backends may also cap the returned entries at a hard-coded limit, the default REST server implementation has such a hard-coded limit.  To implement paging, check 'hasMore' in the response and, if 'true', pass the value returned as 'token' in the next invocation as the 'pageToken' parameter.  The content and meaning of the returned 'token' is \"private\" to the implementation,treat is as an opaque value.  It is wrong to assume that invoking this method with a very high 'maxRecords' value will return all commit log entries.  The 'filter' parameter allows for advanced filtering capabilities using the Common Expression Language (CEL). An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | ref to show log from | [required] |
**end_hash** | Option<**String**> | Hash on the given ref to end at (in chronological sense), the 'near' end of the commit log, returned 'early' in the result. |  |
**fetch** | Option<[**FetchOption**](.md)> | Specify how much information to be returned. Will fetch additional metadata such as parent commit hash and operations in a commit, for each commit if set to 'ALL'. |  |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md.  Usable variables within the expression are:  - 'commit' with fields 'author' (string), 'committer' (string), 'commitTime' (timestamp), 'hash' (string), ',message' (string), 'properties' (map)  - 'operations' (list), each operation has the fields 'type' (string, either 'PUT' or 'DELETE'), 'key' (string, namespace + table name), 'keyElements' (list of strings), 'namespace' (string), 'namespaceElements' (list of strings) and 'name' (string, the \"simple\" table name)  Note that the expression can only test against 'operations', if 'fetch' is set to 'ALL'.  Hint: when filtering commits, you can determine whether commits are \"missing\" (filtered) by checking whether 'LogEntry.parentCommitHash' is different from the hash of the previous commit in the log response. |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |
**start_hash** | Option<**String**> | Hash on the given ref to start from (in chronological sense), the 'far' end of the commit log, returned 'late' in the result. |  |

### Return type

[**crate::models::LogResponseV1**](LogResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config

> crate::models::NessieConfigurationV1 get_config()
List all configuration settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NessieConfigurationV1**](NessieConfiguration_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content

> crate::models::ContentV1 get_content(key, hash_on_ref, r#ref)
Get object content associated with a key.

This operation returns the content-value for a content-key in a named-reference (a branch or tag).  If the table-metadata is tracked globally (Iceberg), Nessie returns a 'Content' object, that contains the most up-to-date part for the globally tracked part (Iceberg: table-metadata) plus the per-Nessie-reference/hash specific part (Iceberg: snapshot-id, schema-id, partition-spec-id, default-sort-order-id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | object name to search for | [required] |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |
**r#ref** | Option<**String**> | Reference to use. Defaults to default branch if not provided. |  |

### Return type

[**crate::models::ContentV1**](Content_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_branch

> crate::models::BranchV1 get_default_branch()
Get default branch for commits and reads

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BranchV1**](Branch_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_diff

> crate::models::DiffResponseV1 get_diff(from_ref_with_hash, to_ref_with_hash)
Get a diff for two given references

The URL pattern is basically 'from' and 'to' separated by '...' (three dots). 'from' and 'to' must start with a reference name, optionally followed by hash on that reference, the hash prefixed with the'*' character.  Examples:    diffs/main...myBranch   diffs/main...myBranch\\*1234567890123456   diffs/main\\*1234567890123456...myBranch   diffs/main\\*1234567890123456...myBranch\\*1234567890123456 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_ref_with_hash** | **String** | The 'from' reference (and optional hash) to start the diff from | [required] |
**to_ref_with_hash** | **String** | The 'to' reference (and optional hash) to end the diff at. | [required] |

### Return type

[**crate::models::DiffResponseV1**](DiffResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entries

> crate::models::EntriesResponseV1 get_entries(r#ref, filter, hash_on_ref, max_records, namespace_depth, page_token)
Fetch all entries for a given reference

Retrieves objects for a ref, potentially truncated by the backend.  Retrieves up to 'maxRecords' entries for the given named reference (tag or branch) or the given hash. The backend may respect the given 'max' records hint, but return less or more entries. Backends may also cap the returned entries at a hard-coded limit, the default REST server implementation has such a hard-coded limit.  To implement paging, check 'hasMore' in the response and, if 'true', pass the value returned as 'token' in the next invocation as the 'pageToken' parameter.  The content and meaning of the returned 'token' is \"private\" to the implementation,treat is as an opaque value.  It is wrong to assume that invoking this method with a very high 'maxRecords' value will return all commit log entries.  The 'filter' parameter allows for advanced filtering capabilities using the Common Expression Language (CEL). An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md.  The 'namespaceDepth' parameter returns only the ContentKey components up to the depth of 'namespaceDepth'. For example they key 'a.b.c.d' with a depth of 3 will return 'a.b.c'. The operation is guaranteed to not return  duplicates and therefore will never page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | name of ref to fetch from | [required] |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md. Usable variables within the expression are 'entry.namespace' (string) & 'entry.contentType' (string) |  |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**namespace_depth** | Option<**i32**> | If set > 0 will filter the results to only return namespaces/tables to the depth of namespaceDepth. If not set or <=0 has no effect Setting this parameter > 0 will turn off paging. |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |

### Return type

[**crate::models::EntriesResponseV1**](EntriesResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multiple_contents

> crate::models::GetMultipleContentsResponseV1 get_multiple_contents(hash_on_ref, r#ref, get_multiple_contents_request)
Get multiple objects' content.

Similar to 'getContent', but takes multiple 'ContentKey's and returns the content-values for the one or more content-keys in a named-reference (a branch or tag).  If the table-metadata is tracked globally (Iceberg), Nessie returns a 'Content' object, that contains the most up-to-date part for the globally tracked part (Iceberg: table-metadata) plus the per-Nessie-reference/hash specific part (Iceberg: snapshot-ID,schema-ID, partition-spec-ID, default-sort-order-ID).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |
**r#ref** | Option<**String**> | Reference to use. Defaults to default branch if not provided. |  |
**get_multiple_contents_request** | Option<[**GetMultipleContentsRequest**](GetMultipleContentsRequest.md)> | Keys to retrieve. |  |

### Return type

[**crate::models::GetMultipleContentsResponseV1**](GetMultipleContentsResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace

> crate::models::NamespaceV1 get_namespace(name, r#ref, hash_on_ref)
Retrieves a Namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**Namespace**](.md) | the name of the namespace | [required] |
**r#ref** | **String** | name of ref to fetch | [required] |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |

### Return type

[**crate::models::NamespaceV1**](Namespace_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespaces

> crate::models::GetNamespacesResponseV1 get_namespaces(r#ref, hash_on_ref, name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | name of ref to fetch | [required] |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |
**name** | Option<[**Namespace**](.md)> | the name of the namespace |  |

### Return type

[**crate::models::GetNamespacesResponseV1**](GetNamespacesResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ref_log

> serde_json::Value get_ref_log(end_hash, filter, max_records, page_token, start_hash)
Get reflog entries (DEPRECATED)

The Nessie reflog in this form is deprecated, likely for removal. Retrieve the reflog entries from a specified endHash or from the current HEAD if the endHash is null, potentially truncated by the backend.  Retrieves up to 'maxRecords' refLog-entries starting at the endHash or HEAD.The backend may respect the given 'max' records hint, but return less or more entries. Backends may also cap the returned entries at a hard-coded limit, the default REST server implementation has such a hard-coded limit.  To implement paging, check 'hasMore' in the response and, if 'true', pass the value returned as 'token' in the next invocation as the 'pageToken' parameter.  The content and meaning of the returned 'token' is \"private\" to the implementation,treat is as an opaque value.  It is wrong to assume that invoking this method with a very high 'maxRecords' value will return all reflog entries.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_hash** | Option<**String**> | Hash of the reflog (inclusive) to end at (in chronological sense), the 'near' end of the reflog, returned 'early' in the result. |  |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md.  Usable variables within the expression are:  - 'reflog' with fields 'refLogId' (string), 'refName' (string), 'commitHash' (string), 'parentRefLogId' (string), ',operation' (string), 'operationTime' (long)  Hint: when filtering entries, you can determine whether entries are \"missing\" (filtered) by checking whether 'ReflogResponseEntry.parentRefLogId' is different from the hash of the previous reflog in the log response. |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |
**start_hash** | Option<**String**> | Hash of the reflog (inclusive) to start from (in chronological sense), the 'far' end of the reflog, returned 'late' in the result. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reference_by_name

> crate::models::ReferenceV1 get_reference_by_name(r#ref, fetch)
Fetch details of a reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | name of ref to fetch | [required] |
**fetch** | Option<[**FetchOption**](.md)> | Specify how much information to be returned. Will fetch additional metadata for references if set to 'ALL'.  A returned Branch instance will have the following information:  - numCommitsAhead (number of commits ahead of the default branch)  - numCommitsBehind (number of commits behind the default branch)  - commitMetaOfHEAD (the commit metadata of the HEAD commit)  - commonAncestorHash (the hash of the common ancestor in relation to the default branch).  - numTotalCommits (the total number of commits in this reference).  A returned Tag instance will only contain the 'commitMetaOfHEAD' and 'numTotalCommits' fields.  Note that computing & fetching additional metadata might be computationally expensive on the server-side, so this flag should be used with care. |  |

### Return type

[**crate::models::ReferenceV1**](Reference_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_ref_into_branch

> crate::models::MergeResponseV1 merge_ref_into_branch(branch_name, expected_hash, merge1)
Merge commits from 'mergeRef' onto 'branchName'.

Merge items from an existing hash in 'mergeRef' into the requested branch. The merge is always a rebase + fast-forward merge and is only completed if the rebase is conflict free. The set of commits added to the branch will be all of those until we arrive at a common ancestor. Depending on the underlying implementation, the number of commits allowed as part of this operation may be limited.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch_name** | **String** | Branch to merge into | [required] |
**expected_hash** | **String** | Expected current HEAD of 'branchName' | [required] |
**merge1** | Option<[**Merge1**](Merge1.md)> | Merge operation that defines the source reference name and an optional hash. If 'fromHash' is not present, the current 'sourceRef's HEAD will be used. |  |

### Return type

[**crate::models::MergeResponseV1**](MergeResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transplant_commits_into_branch

> crate::models::MergeResponseV1 transplant_commits_into_branch(branch_name, expected_hash, message, transplant1)
Transplant commits from 'transplant' onto 'branchName'

This is done as an atomic operation such that only the last of the sequence is ever visible to concurrent readers/writers. The sequence to transplant must be contiguous and in order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch_name** | **String** | Branch to transplant into | [required] |
**expected_hash** | **String** | Expected hash of tag. | [required] |
**message** | Option<**String**> | commit message |  |
**transplant1** | Option<[**Transplant1**](Transplant1.md)> | Hashes to transplant |  |

### Return type

[**crate::models::MergeResponseV1**](MergeResponse_V1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_properties

> update_properties(name, r#ref, hash_on_ref, namespace_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**Namespace**](.md) | the name of the namespace | [required] |
**r#ref** | **String** | name of ref to fetch | [required] |
**hash_on_ref** | Option<**String**> | a particular hash on the given ref |  |
**namespace_update** | Option<[**NamespaceUpdate**](NamespaceUpdate.md)> | Namespace properties to update/delete. |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

