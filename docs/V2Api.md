# \V2Api

All URIs are relative to *http://localhost:19120/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_reference_v2**](V2Api.md#assign_reference_v2) | **PUT** /v2/trees/{ref} | Set a named reference to a specific hash via another reference.
[**commit_v2**](V2Api.md#commit_v2) | **POST** /v2/trees/{branch}/history/commit | Commit one or more operations against the given 'branch'.
[**create_reference_v2**](V2Api.md#create_reference_v2) | **POST** /v2/trees | Create a new branch or tag
[**delete_reference_v2**](V2Api.md#delete_reference_v2) | **DELETE** /v2/trees/{ref} | Delete a reference
[**get_all_references_v2**](V2Api.md#get_all_references_v2) | **GET** /v2/trees | Get information about all branches and tags
[**get_commit_log_v2**](V2Api.md#get_commit_log_v2) | **GET** /v2/trees/{ref}/history | Get commit log for a particular reference
[**get_config_v2**](V2Api.md#get_config_v2) | **GET** /v2/config | Returns repository and server settings relevant to clients.
[**get_content_v2**](V2Api.md#get_content_v2) | **GET** /v2/trees/{ref}/contents/{key} | Get the content object associated with a key.
[**get_diff_v2**](V2Api.md#get_diff_v2) | **GET** /v2/trees/{from-ref}/diff/{to-ref} | Get contents that differ in the trees specified by the two given references
[**get_entries_v2**](V2Api.md#get_entries_v2) | **GET** /v2/trees/{ref}/entries | Fetch all entries for a given reference
[**get_multiple_contents_v2**](V2Api.md#get_multiple_contents_v2) | **POST** /v2/trees/{ref}/contents | Get multiple content objects.
[**get_reference_by_name_v2**](V2Api.md#get_reference_by_name_v2) | **GET** /v2/trees/{ref} | Fetch details of a reference
[**get_repository_config**](V2Api.md#get_repository_config) | **GET** /v2/config/repository | Returns repository configurations of the requested types.
[**get_several_contents**](V2Api.md#get_several_contents) | **GET** /v2/trees/{ref}/contents | Get multiple content objects.
[**merge_v2**](V2Api.md#merge_v2) | **POST** /v2/trees/{branch}/history/merge | Merge commits from another reference onto 'branch'.
[**transplant_v2**](V2Api.md#transplant_v2) | **POST** /v2/trees/{branch}/history/transplant | Transplant commits specified by the 'Transplant' payload object onto the given 'branch'
[**update_repository_config**](V2Api.md#update_repository_config) | **POST** /v2/config/repository | Create or update a repository configuration.



## assign_reference_v2

> crate::models::SingleReferenceResponseV2 assign_reference_v2(r#ref, reference, r#type)
Set a named reference to a specific hash via another reference.

The 'ref' parameter identifies the branch or tag to be reassigned. The 'ref' parameter may contain a hash qualifier. That hash as well as the optional 'type' parameter may be used to ensure the operation is performed on the same object that the user expects.  Only branches and tags may be reassigned. The payload object identifies any reference visible to the current user whose 'hash' will be used to define the new HEAD of the reference being reassigned. Detached hashes may be used in the payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | [**serde_json::Value**](.md) | Specifies a named branch or tag reference with its expected HEAD 'hash' value.  For example: - name@hash - Identifies the 'hash' commit on a branch or tag.  The specified 'hash' must be the value of the current HEAD of the branch or tag known by the client. It will be used to validate that at execution time the reference points to the same hash that the caller expected when the operation arguments were constructed.  | [required] |
**reference** | [**Reference**](Reference.md) | Reference to which the 'ref' (from the path parameter) shall be assigned. This must be either a 'Detached' commit, 'Branch' or 'Tag' via which the hash is visible to the caller. | [required] |
**r#type** | Option<[**ReferenceType**](.md)> | Optional expected type of the reference being reassigned |  |

### Return type

[**crate::models::SingleReferenceResponseV2**](SingleReferenceResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_v2

> crate::models::CommitResponseV2 commit_v2(branch, operations)
Commit one or more operations against the given 'branch'.

The state of contents specified by the 'branch' reference will be used for detecting conflicts with the operation being committed.  The hash in the successful response will be the hash of the commit that contains the requested operations, whose immediate parent commit will be the current HEAD of the specified branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch** | [**serde_json::Value**](.md) | A reference to a particular version of the contents tree (a point in history) on a branch. This reference is specified in this form: - name@hash - Identifies the 'hash' commit on the named branch.  The 'hash' commit must be reachable from the current HEAD of the branch. In this case 'hash' indicates the state of contents that should be used for validating incoming changes.  | [required] |
**operations** | [**Operations**](Operations.md) | Operations to commit | [required] |

### Return type

[**crate::models::CommitResponseV2**](CommitResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reference_v2

> crate::models::SingleReferenceResponseV2 create_reference_v2(name, r#type, reference)
Create a new branch or tag

The name and type query parameters define the kind of reference to be created. The payload object defines the new reference's origin in the commit history.  Only branches and tags can be created by this method, but the payload object may be any valid reference, including a detached commit. If the payload reference object does not define a commit hash, the HEAD of that reference will be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | A reference name.  Reference name must start with a letter, followed by letters, digits, one of the ./_- characters, not end with a slash or dot, not contain '..'  | [required] |
**r#type** | [**ReferenceType**](.md) | Type of the reference to be created | [required] |
**reference** | [**Reference**](Reference.md) | Source reference data from which the new reference is to be created. | [required] |

### Return type

[**crate::models::SingleReferenceResponseV2**](SingleReferenceResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_reference_v2

> crate::models::SingleReferenceResponseV2 delete_reference_v2(r#ref, r#type)
Delete a reference

The 'ref' parameter identifies the branch or tag to be deleted. The 'ref' parameter may contain a hash qualifier. That hash as well as the optional 'type' parameter may be used to ensure the operation is performed on the same object that the user expects.  Only branches and tags can be deleted. However, deleting the default branch may be restricted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | [**serde_json::Value**](.md) | Specifies a named branch or tag reference with its expected HEAD 'hash' value.  For example: - name@hash - Identifies the 'hash' commit on a branch or tag.  The specified 'hash' must be the value of the current HEAD of the branch or tag known by the client. It will be used to validate that at execution time the reference points to the same hash that the caller expected when the operation arguments were constructed.  | [required] |
**r#type** | Option<[**ReferenceType**](.md)> | Optional expected type of the reference being deleted |  |

### Return type

[**crate::models::SingleReferenceResponseV2**](SingleReferenceResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_references_v2

> crate::models::ReferencesResponseV2 get_all_references_v2(fetch, filter, max_records, page_token)
Get information about all branches and tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fetch** | Option<[**FetchOption**](.md)> | Specifies how much extra information is to be retrived from the server.  If the fetch option is set to 'ALL' the following addition information will be returned for each Branch object in the output:  - numCommitsAhead (number of commits ahead of the default branch)  - numCommitsBehind (number of commits behind the default branch)  - commitMetaOfHEAD (the commit metadata of the HEAD commit)  - commonAncestorHash (the hash of the common ancestor in relation to the default branch).  - numTotalCommits (the total number of commits from the root to the HEAD of the branch).  The returned Tag instances will only contain the 'commitMetaOfHEAD' and 'numTotalCommits' fields.  Note that computing & fetching additional metadata might be computationally expensive on the server-side, so this flag should be used with care. |  |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md. Usable variables within the expression are:  - ref (Reference) describes the reference, with fields name (String), hash (String), metadata (ReferenceMetadata)  - metadata (ReferenceMetadata) shortcut to ref.metadata, never null, but possibly empty  - commit (CommitMeta) - shortcut to ref.metadata.commitMetaOfHEAD, never null, but possibly empty  - refType (String) - the reference type, either BRANCH or TAG  Note that the expression can only test attributes metadata and commit, if 'fetchOption' is set to 'ALL'. |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |

### Return type

[**crate::models::ReferencesResponseV2**](ReferencesResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_commit_log_v2

> crate::models::LogResponseV2 get_commit_log_v2(r#ref, fetch, filter, limit_hash, max_records, page_token)
Get commit log for a particular reference

Retrieve the commit log for a reference, potentially truncated by the backend.  The backend may respect the given 'max-entries' records hint, or may return more or less entries. Backends may also cap the returned entries at a hard-coded limit  To implement paging, check 'hasMore' in the response and, if 'true', pass the value returned as 'token' in the next invocation as the 'pageToken' parameter.  The content and meaning of the returned 'token' is \"private\" to the implementation,treat is as an opaque value.  It is wrong to assume that invoking this method with a very high 'maxRecords' value will return all available data in one page.  Different pages may have different numbers of log records in them even if they come from another call to this method with the same parameters. Also, pages are not guaranteed to be filled to contain exactly 'maxRecords' even if the total amount of available data allows that. Pages may contain more of less entries at server's discretion.  The 'filter' parameter allows for advanced filtering capabilities using the Common Expression Language (CEL). An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md.  The fetching of the log starts from the HEAD of the given ref (or a more specific commit, if provided as part of the 'ref' path element) and proceeds until the 'root' commit or the 'limit-hash' commit are encountered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | [**serde_json::Value**](.md) | A reference to a particular version of the contents tree (a point in history).  Reference representations consist of: - The reference name. '-' means the default branch name. - A commit hash prefixed with '@'. - A relative commit specification. '~N' means the N-th predecessor commit, '*T' means the commit for which the timestamp T (milliseconds since epoch or ISO-8601 instant) is valid, '^N' means the N-th parentin a commit (N=2 is the merge parent).  If neither the reference name or the default branch name placeholder '-' is specified, the reference type 'DETACHED' will be assumed. If no commit hash is specified, the HEAD of the specified named reference will be used. An empty reference parameter is not valid.  This reference can be specified in these forms: - \\- (literal minus character) - identifies the HEAD of the default branch. - name - Identifies the HEAD commit of a branch or tag. - name@hash - Identifies the 'hash' commit on a branch or tag. - @hash - Identifies the 'hash' commit in an unspecified branch or tag. - -~3 - The 3rd predecessor commit from the HEAD of the default branch. - name~3 - The 3rd predecessor commit from the HEAD of a branch or tag. - @hash~3 - The 3rd predecessor commit of the 'hash' commit. - name@hash^2 - The merge parent of the 'hash' commit of a branch or tag. - @hash^2 - The merge parent of the 'hash' commit. - -*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of the default branch for the given ISO-8601 timestamp. - name*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of a branch or tag valid for the given ISO-8601 timestamp. - name*1685185847230 - The predecessor commit closest to the HEAD of a branch or tag valid for the given timestamp in milliseconds since epoch.  If both 'name' and 'hash' are given, 'hash' must be reachable from the current HEAD of the branch or tag. If 'name' is omitted, the reference will be of type 'DETACHED' (referencing a specific commit hash without claiming its reachability from any live HEAD). Using references of the last form may have authorization implications when compared to an equivalent reference of the former forms.  An empty reference parameter is invalid.  The 'name@hash' form always refers to the exact commit on a specific named reference. This is the most complete form of a reference. Other forms omit some of the details and require those gaps to be filled by the server at runtime. Although these forms may be convenient to a human-being, they may resolve differently at different times depending on the state of the system. Using the full 'name@hash' form is recommended to avoid ambiguity.  | [required] |
**fetch** | Option<[**FetchOption**](.md)> | Specify how much information to be returned. Will fetch additional metadata such as parent commit hash and operations in a commit, for each commit if set to 'ALL'. |  |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md.  Usable variables within the expression are:  - 'commit' with fields 'author' (string), 'committer' (string), 'commitTime' (timestamp), 'hash' (string), ',message' (string), 'properties' (map)  - 'operations' (list), each operation has the fields 'type' (string, either 'PUT' or 'DELETE'), 'key' (string, namespace + table name), 'keyElements' (list of strings), 'namespace' (string), 'namespaceElements' (list of strings) and 'name' (string, the \"simple\" table name)  Note that the expression can only test against 'operations', if 'fetch' is set to 'ALL'.  Hint: when filtering commits, you can determine whether commits are \"missing\" (filtered) by checking whether 'LogEntry.parentCommitHash' is different from the hash of the previous commit in the log response. |  |
**limit_hash** | Option<**String**> | Hash on the given ref to identify the commit where the operation of fetching the log should stop, i.e. the 'far' end of the commit log, returned late in the result. |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |

### Return type

[**crate::models::LogResponseV2**](LogResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_v2

> crate::models::NessieConfigurationV2 get_config_v2()
Returns repository and server settings relevant to clients.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NessieConfigurationV2**](NessieConfiguration_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_v2

> crate::models::ContentResponseV2 get_content_v2(key, r#ref, with_doc)
Get the content object associated with a key.

This operation returns the content value for a content key at a particular point in history as defined by the 'ref' parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | [**ContentKey**](.md) | The key to a content object.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  | [required] |
**r#ref** | [**serde_json::Value**](.md) | A reference to a particular version of the contents tree (a point in history).  Reference representations consist of: - The reference name. '-' means the default branch name. - A commit hash prefixed with '@'. - A relative commit specification. '~N' means the N-th predecessor commit, '*T' means the commit for which the timestamp T (milliseconds since epoch or ISO-8601 instant) is valid, '^N' means the N-th parentin a commit (N=2 is the merge parent).  If neither the reference name or the default branch name placeholder '-' is specified, the reference type 'DETACHED' will be assumed. If no commit hash is specified, the HEAD of the specified named reference will be used. An empty reference parameter is not valid.  This reference can be specified in these forms: - \\- (literal minus character) - identifies the HEAD of the default branch. - name - Identifies the HEAD commit of a branch or tag. - name@hash - Identifies the 'hash' commit on a branch or tag. - @hash - Identifies the 'hash' commit in an unspecified branch or tag. - -~3 - The 3rd predecessor commit from the HEAD of the default branch. - name~3 - The 3rd predecessor commit from the HEAD of a branch or tag. - @hash~3 - The 3rd predecessor commit of the 'hash' commit. - name@hash^2 - The merge parent of the 'hash' commit of a branch or tag. - @hash^2 - The merge parent of the 'hash' commit. - -*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of the default branch for the given ISO-8601 timestamp. - name*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of a branch or tag valid for the given ISO-8601 timestamp. - name*1685185847230 - The predecessor commit closest to the HEAD of a branch or tag valid for the given timestamp in milliseconds since epoch.  If both 'name' and 'hash' are given, 'hash' must be reachable from the current HEAD of the branch or tag. If 'name' is omitted, the reference will be of type 'DETACHED' (referencing a specific commit hash without claiming its reachability from any live HEAD). Using references of the last form may have authorization implications when compared to an equivalent reference of the former forms.  An empty reference parameter is invalid.  The 'name@hash' form always refers to the exact commit on a specific named reference. This is the most complete form of a reference. Other forms omit some of the details and require those gaps to be filled by the server at runtime. Although these forms may be convenient to a human-being, they may resolve differently at different times depending on the state of the system. Using the full 'name@hash' form is recommended to avoid ambiguity.  | [required] |
**with_doc** | Option<**bool**> | Whether to return the documentation, if it exists. Default is to not return the documentation. |  |

### Return type

[**crate::models::ContentResponseV2**](ContentResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_diff_v2

> crate::models::DiffResponseV2 get_diff_v2(from_ref, to_ref, filter, key, max_key, max_records, min_key, page_token, prefix_key)
Get contents that differ in the trees specified by the two given references

The URL pattern is basically 'from' and 'to' reference specs separated by '/diff/'  Examples:  - main/diff/myBranch - main@1234567890123456/diff/myBranch - main@1234567890123456/diff/myBranch@23445678 - main/diff/myBranch@23445678 - main/diff/myBranch@23445678 - my/branch@/diff/main - myBranch/diff/- 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_ref** | **String** | A reference to a particular version of the contents tree (a point in history).  Reference representations consist of: - The reference name. '-' means the default branch name. - A commit hash prefixed with '@'. - A relative commit specification. '~N' means the N-th predecessor commit, '*T' means the commit for which the timestamp T (milliseconds since epoch or ISO-8601 instant) is valid, '^N' means the N-th parentin a commit (N=2 is the merge parent).  If neither the reference name or the default branch name placeholder '-' is specified, the reference type 'DETACHED' will be assumed. If no commit hash is specified, the HEAD of the specified named reference will be used. An empty reference parameter is not valid.  This reference can be specified in these forms: - \\- (literal minus character) - identifies the HEAD of the default branch. - name - Identifies the HEAD commit of a branch or tag. - name@hash - Identifies the 'hash' commit on a branch or tag. - @hash - Identifies the 'hash' commit in an unspecified branch or tag. - -~3 - The 3rd predecessor commit from the HEAD of the default branch. - name~3 - The 3rd predecessor commit from the HEAD of a branch or tag. - @hash~3 - The 3rd predecessor commit of the 'hash' commit. - name@hash^2 - The merge parent of the 'hash' commit of a branch or tag. - @hash^2 - The merge parent of the 'hash' commit. - -*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of the default branch for the given ISO-8601 timestamp. - name*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of a branch or tag valid for the given ISO-8601 timestamp. - name*1685185847230 - The predecessor commit closest to the HEAD of a branch or tag valid for the given timestamp in milliseconds since epoch.  If both 'name' and 'hash' are given, 'hash' must be reachable from the current HEAD of the branch or tag. If 'name' is omitted, the reference will be of type 'DETACHED' (referencing a specific commit hash without claiming its reachability from any live HEAD). Using references of the last form may have authorization implications when compared to an equivalent reference of the former forms.  An empty reference parameter is invalid.  The 'name@hash' form always refers to the exact commit on a specific named reference. This is the most complete form of a reference. Other forms omit some of the details and require those gaps to be filled by the server at runtime. Although these forms may be convenient to a human-being, they may resolve differently at different times depending on the state of the system. Using the full 'name@hash' form is recommended to avoid ambiguity.  | [required] |
**to_ref** | **String** | Same reference spec as in the 'from-ref' parameter but identifying the other tree for comparison. | [required] |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md.  Usable variables within the expression are:  - 'key' (string, namespace + table name), 'keyElements' (list of strings), 'namespace' (string), 'namespaceElements' (list of strings) and 'name' (string, the \"simple\" table name) |  |
**key** | Option<[**Vec<crate::models::ContentKey>**](crate::models::ContentKey.md)> | Restrict the result to one or more keys.  Can be combined with min/max-key and prefix-key parameters, however both predicates must match. This means that keys specified via this parameter that do not match a given min/max-key or prefix-key will not be returned.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**max_key** | Option<[**ContentKey**](.md)> | The upper bound of the content key range to retrieve (inclusive). The content keys of all returned entries will be less than or equal to the max-value. Content-keys are compared as a 'whole', unlike prefix-keys.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**min_key** | Option<[**ContentKey**](.md)> | The lower bound of the content key range to retrieve (inclusive). The content keys of all returned entries will be greater than or equal to the min-value. Content-keys are compared as a 'whole', unlike prefix-keys.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |
**prefix_key** | Option<[**ContentKey**](.md)> | The content key prefix to retrieve (inclusive). A content key matches a given prefix, a content key's elements starts with all elements of the prefix-key. Key prefixes exactly match key-element boundaries.  Must not be combined with min/max-key parameters.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |

### Return type

[**crate::models::DiffResponseV2**](DiffResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entries_v2

> crate::models::EntriesResponseV2 get_entries_v2(r#ref, content, filter, key, max_key, max_records, min_key, page_token, prefix_key)
Fetch all entries for a given reference

Retrieves objects for a ref, potentially truncated by the backend.  Retrieves up to 'maxRecords' entries for the given named reference (tag or branch) or the given hash. The backend may respect the given 'max' records hint, but return less or more entries. Backends may also cap the returned entries at a hard-coded limit, the default REST server implementation has such a hard-coded limit.  To implement paging, check 'hasMore' in the response and, if 'true', pass the value returned as 'token' in the next invocation as the 'pageToken' parameter.  The content and meaning of the returned 'token' is \"private\" to the implementation,treat is as an opaque value.  It is wrong to assume that invoking this method with a very high 'maxRecords' value will return all available data in one page.  Different pages may have different numbers of log records in them even if they come from another call to this method with the same parameters. Also, pages are not guaranteed to be filled to contain exactly 'maxRecords' even if the total amount of available data allows that. Pages may contain more of less entries at server's discretion.  The 'filter' parameter allows for advanced filtering capabilities using the Common Expression Language (CEL). An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | [**serde_json::Value**](.md) | A reference to a particular version of the contents tree (a point in history).  Reference representations consist of: - The reference name. '-' means the default branch name. - A commit hash prefixed with '@'. - A relative commit specification. '~N' means the N-th predecessor commit, '*T' means the commit for which the timestamp T (milliseconds since epoch or ISO-8601 instant) is valid, '^N' means the N-th parentin a commit (N=2 is the merge parent).  If neither the reference name or the default branch name placeholder '-' is specified, the reference type 'DETACHED' will be assumed. If no commit hash is specified, the HEAD of the specified named reference will be used. An empty reference parameter is not valid.  This reference can be specified in these forms: - \\- (literal minus character) - identifies the HEAD of the default branch. - name - Identifies the HEAD commit of a branch or tag. - name@hash - Identifies the 'hash' commit on a branch or tag. - @hash - Identifies the 'hash' commit in an unspecified branch or tag. - -~3 - The 3rd predecessor commit from the HEAD of the default branch. - name~3 - The 3rd predecessor commit from the HEAD of a branch or tag. - @hash~3 - The 3rd predecessor commit of the 'hash' commit. - name@hash^2 - The merge parent of the 'hash' commit of a branch or tag. - @hash^2 - The merge parent of the 'hash' commit. - -*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of the default branch for the given ISO-8601 timestamp. - name*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of a branch or tag valid for the given ISO-8601 timestamp. - name*1685185847230 - The predecessor commit closest to the HEAD of a branch or tag valid for the given timestamp in milliseconds since epoch.  If both 'name' and 'hash' are given, 'hash' must be reachable from the current HEAD of the branch or tag. If 'name' is omitted, the reference will be of type 'DETACHED' (referencing a specific commit hash without claiming its reachability from any live HEAD). Using references of the last form may have authorization implications when compared to an equivalent reference of the former forms.  An empty reference parameter is invalid.  The 'name@hash' form always refers to the exact commit on a specific named reference. This is the most complete form of a reference. Other forms omit some of the details and require those gaps to be filled by the server at runtime. Although these forms may be convenient to a human-being, they may resolve differently at different times depending on the state of the system. Using the full 'name@hash' form is recommended to avoid ambiguity.  | [required] |
**content** | Option<**bool**> | Optionally request to return 'Content' objects for the returned keys. |  |
**filter** | Option<**String**> | A Common Expression Language (CEL) expression. An intro to CEL can be found at https://github.com/google/cel-spec/blob/master/doc/intro.md. Usable variables within the expression are 'entry.namespace' (string) & 'entry.contentType' (string) |  |
**key** | Option<[**Vec<crate::models::ContentKey>**](crate::models::ContentKey.md)> | Restrict the result to one or more keys.  Can be combined with min/max-key and prefix-key parameters, however both predicates must match. This means that keys specified via this parameter that do not match a given min/max-key or prefix-key will not be returned.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**max_key** | Option<[**ContentKey**](.md)> | The upper bound of the content key range to retrieve (inclusive). The content keys of all returned entries will be less than or equal to the max-value. Content-keys are compared as a 'whole', unlike prefix-keys.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**max_records** | Option<**i32**> | maximum number of entries to return, just a hint for the server |  |
**min_key** | Option<[**ContentKey**](.md)> | The lower bound of the content key range to retrieve (inclusive). The content keys of all returned entries will be greater than or equal to the min-value. Content-keys are compared as a 'whole', unlike prefix-keys.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**page_token** | Option<**String**> | paging continuation token, as returned in the previous value of the field 'token' in the corresponding 'EntriesResponse' or 'LogResponse' or 'ReferencesResponse' or 'RefLogResponse'. |  |
**prefix_key** | Option<[**ContentKey**](.md)> | The content key prefix to retrieve (inclusive). A content key matches a given prefix, a content key's elements starts with all elements of the prefix-key. Key prefixes exactly match key-element boundaries.  Must not be combined with min/max-key parameters.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |

### Return type

[**crate::models::EntriesResponseV2**](EntriesResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multiple_contents_v2

> crate::models::GetMultipleContentsResponseV2 get_multiple_contents_v2(r#ref, with_doc, get_multiple_contents_request)
Get multiple content objects.

Similar to 'GET /trees/{ref}/content/{key}', but takes multiple 'ContentKey's (in the JSON payload) and returns zero or more content objects.  Note that if some keys from the request do not have an associated content object at the point in history defined by the 'ref' parameter, the response will be successful, but no data will be returned for the missing keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | [**serde_json::Value**](.md) | A reference to a particular version of the contents tree (a point in history).  Reference representations consist of: - The reference name. '-' means the default branch name. - A commit hash prefixed with '@'. - A relative commit specification. '~N' means the N-th predecessor commit, '*T' means the commit for which the timestamp T (milliseconds since epoch or ISO-8601 instant) is valid, '^N' means the N-th parentin a commit (N=2 is the merge parent).  If neither the reference name or the default branch name placeholder '-' is specified, the reference type 'DETACHED' will be assumed. If no commit hash is specified, the HEAD of the specified named reference will be used. An empty reference parameter is not valid.  This reference can be specified in these forms: - \\- (literal minus character) - identifies the HEAD of the default branch. - name - Identifies the HEAD commit of a branch or tag. - name@hash - Identifies the 'hash' commit on a branch or tag. - @hash - Identifies the 'hash' commit in an unspecified branch or tag. - -~3 - The 3rd predecessor commit from the HEAD of the default branch. - name~3 - The 3rd predecessor commit from the HEAD of a branch or tag. - @hash~3 - The 3rd predecessor commit of the 'hash' commit. - name@hash^2 - The merge parent of the 'hash' commit of a branch or tag. - @hash^2 - The merge parent of the 'hash' commit. - -*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of the default branch for the given ISO-8601 timestamp. - name*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of a branch or tag valid for the given ISO-8601 timestamp. - name*1685185847230 - The predecessor commit closest to the HEAD of a branch or tag valid for the given timestamp in milliseconds since epoch.  If both 'name' and 'hash' are given, 'hash' must be reachable from the current HEAD of the branch or tag. If 'name' is omitted, the reference will be of type 'DETACHED' (referencing a specific commit hash without claiming its reachability from any live HEAD). Using references of the last form may have authorization implications when compared to an equivalent reference of the former forms.  An empty reference parameter is invalid.  The 'name@hash' form always refers to the exact commit on a specific named reference. This is the most complete form of a reference. Other forms omit some of the details and require those gaps to be filled by the server at runtime. Although these forms may be convenient to a human-being, they may resolve differently at different times depending on the state of the system. Using the full 'name@hash' form is recommended to avoid ambiguity.  | [required] |
**with_doc** | Option<**bool**> | Whether to return the documentation, if it exists. Default is to not return the documentation. |  |
**get_multiple_contents_request** | Option<[**GetMultipleContentsRequest**](GetMultipleContentsRequest.md)> | Keys to retrieve. |  |

### Return type

[**crate::models::GetMultipleContentsResponseV2**](GetMultipleContentsResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reference_by_name_v2

> crate::models::SingleReferenceResponseV2 get_reference_by_name_v2(r#ref, fetch)
Fetch details of a reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | Specifies a reference to a particular commit history branch or tag.  This reference can be specification in these forms: - \\- (literal minus character) - identifies the default branch. - name - Identifies the named branch or tag.  | [required] |
**fetch** | Option<[**FetchOption**](.md)> | Specify how much information to be returned. Will fetch additional metadata for references if set to 'ALL'.  A returned Branch instance will have the following information:  - numCommitsAhead (number of commits ahead of the default branch)  - numCommitsBehind (number of commits behind the default branch)  - commitMetaOfHEAD (the commit metadata of the HEAD commit)  - commonAncestorHash (the hash of the common ancestor in relation to the default branch).  - numTotalCommits (the total number of commits in this reference).  A returned Tag instance will only contain the 'commitMetaOfHEAD' and 'numTotalCommits' fields.  Note that computing & fetching additional metadata might be computationally expensive on the server-side, so this flag should be used with care. |  |

### Return type

[**crate::models::SingleReferenceResponseV2**](SingleReferenceResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_config

> crate::models::RepositoryConfigurationObjectsForTheRequestedTypes get_repository_config(r#type)
Returns repository configurations of the requested types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::RepositoryConfigurationObjectsForTheRequestedTypes**](Repository_configuration_objects_for_the_requested_types_.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_several_contents

> crate::models::GetMultipleContentsResponseV2 get_several_contents(r#ref, key, with_doc)
Get multiple content objects.

Similar to 'GET /trees/{ref}/content/{key}', but takes multiple 'key' query parameters and returns zero or more content values in the same JSON structure as the 'POST /trees/{ref}/content' endpoint.  This is a convenience method for fetching a small number of content objects. It is mostly intended for human use. For automated use cases or when the number of keys is large the 'POST /trees/{ref}/content' method is preferred.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | A reference to a particular version of the contents tree (a point in history).  Reference representations consist of: - The reference name. '-' means the default branch name. - A commit hash prefixed with '@'. - A relative commit specification. '~N' means the N-th predecessor commit, '*T' means the commit for which the timestamp T (milliseconds since epoch or ISO-8601 instant) is valid, '^N' means the N-th parentin a commit (N=2 is the merge parent).  If neither the reference name or the default branch name placeholder '-' is specified, the reference type 'DETACHED' will be assumed. If no commit hash is specified, the HEAD of the specified named reference will be used. An empty reference parameter is not valid.  This reference can be specified in these forms: - \\- (literal minus character) - identifies the HEAD of the default branch. - name - Identifies the HEAD commit of a branch or tag. - name@hash - Identifies the 'hash' commit on a branch or tag. - @hash - Identifies the 'hash' commit in an unspecified branch or tag. - -~3 - The 3rd predecessor commit from the HEAD of the default branch. - name~3 - The 3rd predecessor commit from the HEAD of a branch or tag. - @hash~3 - The 3rd predecessor commit of the 'hash' commit. - name@hash^2 - The merge parent of the 'hash' commit of a branch or tag. - @hash^2 - The merge parent of the 'hash' commit. - -*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of the default branch for the given ISO-8601 timestamp. - name*2021-04-07T14:42:25.534748Z - The predecessor commit closest to the HEAD of a branch or tag valid for the given ISO-8601 timestamp. - name*1685185847230 - The predecessor commit closest to the HEAD of a branch or tag valid for the given timestamp in milliseconds since epoch.  If both 'name' and 'hash' are given, 'hash' must be reachable from the current HEAD of the branch or tag. If 'name' is omitted, the reference will be of type 'DETACHED' (referencing a specific commit hash without claiming its reachability from any live HEAD). Using references of the last form may have authorization implications when compared to an equivalent reference of the former forms.  An empty reference parameter is invalid.  The 'name@hash' form always refers to the exact commit on a specific named reference. This is the most complete form of a reference. Other forms omit some of the details and require those gaps to be filled by the server at runtime. Although these forms may be convenient to a human-being, they may resolve differently at different times depending on the state of the system. Using the full 'name@hash' form is recommended to avoid ambiguity.  | [required] |
**key** | Option<[**Vec<String>**](String.md)> | The key to a content object.  Key components (namespaces) are separated by the dot ('.') character. Dot ('.') characters that are not Nessie namespace separators must be encoded as the 'group separator' ASCII character (0x1D).  |  |
**with_doc** | Option<**bool**> | Whether to return the documentation, if it exists. Default is to not return the documentation. |  |

### Return type

[**crate::models::GetMultipleContentsResponseV2**](GetMultipleContentsResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_v2

> crate::models::MergeResponseV2 merge_v2(branch, merge)
Merge commits from another reference onto 'branch'.

Merge commits referenced by the 'mergeRefName' and 'fromHash' parameters of the payload object into the requested 'branch'.  The state of contents specified by the 'branch' reference will be used for detecting conflicts with the commits being transplanted.  The merge is committed if it is free from conflicts. The set of commits merged into the target branch will be all of those starting at 'fromHash' on 'mergeRefName' until we arrive at the common ancestor. Depending on the underlying implementation, the number of commits allowed as part of this operation may be limited.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch** | [**serde_json::Value**](.md) | A reference to a specific version of the contents tree (a point in history) on a branch. This reference is specified in this form: - name@hash - Identifies the 'hash' commit on the named branch.  The 'hash' commit must be reachable from the current HEAD of the branch. In this case 'hash' indicates the state of contents known to the client and serves to ensure that the operation is performed on the contents that the client expects. This hash can point to a commit in the middle of the change history, but it should be as recent as possible.  | [required] |
**merge** | [**Merge**](Merge.md) | Merge operation that defines the source reference name and an optional hash. If 'fromHash' is not present, the current 'sourceRef's HEAD will be used. | [required] |

### Return type

[**crate::models::MergeResponseV2**](MergeResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transplant_v2

> crate::models::MergeResponseV2 transplant_v2(branch, transplant)
Transplant commits specified by the 'Transplant' payload object onto the given 'branch'

This is done as an atomic operation such that only the last of the sequence is ever visible to concurrent readers/writers. The sequence to transplant must be contiguous and in order.  The state of contents specified by the 'branch' reference will be used for detecting conflicts with the commits being transplanted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branch** | [**serde_json::Value**](.md) | A reference to a specific version of the contents tree (a point in history) on a branch. This reference is specified in this form: - name@hash - Identifies the 'hash' commit on the named branch.  The 'hash' commit must be reachable from the current HEAD of the branch. In this case 'hash' indicates the state of contents known to the client and serves to ensure that the operation is performed on the contents that the client expects. This hash can point to a commit in the middle of the change history, but it should be as recent as possible.  | [required] |
**transplant** | [**Transplant**](Transplant.md) | Commits to transplant | [required] |

### Return type

[**crate::models::MergeResponseV2**](MergeResponse_V2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository_config

> crate::models::ThePreviousStateOfTheRepositoryConfigurationObject update_repository_config()
Create or update a repository configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ThePreviousStateOfTheRepositoryConfigurationObject**](The_previous_state_of_the_repository_configuration_object_.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

