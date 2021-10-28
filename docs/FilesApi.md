# \FilesApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_files_copy**](FilesApi.md#drive_files_copy) | **POST** /files/{fileId}/copy | 
[**drive_files_create**](FilesApi.md#drive_files_create) | **POST** /files | 
[**drive_files_delete**](FilesApi.md#drive_files_delete) | **DELETE** /files/{fileId} | 
[**drive_files_empty_trash**](FilesApi.md#drive_files_empty_trash) | **DELETE** /files/trash | 
[**drive_files_export**](FilesApi.md#drive_files_export) | **GET** /files/{fileId}/export | 
[**drive_files_generate_ids**](FilesApi.md#drive_files_generate_ids) | **GET** /files/generateIds | 
[**drive_files_get**](FilesApi.md#drive_files_get) | **GET** /files/{fileId} | 
[**drive_files_list**](FilesApi.md#drive_files_list) | **GET** /files | 
[**drive_files_update**](FilesApi.md#drive_files_update) | **PATCH** /files/{fileId} | 
[**drive_files_watch**](FilesApi.md#drive_files_watch) | **POST** /files/{fileId}/watch | 



## drive_files_copy

> crate::models::File drive_files_copy(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, enforce_single_parent, ignore_default_visibility, include_permissions_for_view, keep_revision_forever, ocr_language, supports_all_drives, supports_team_drives, file)


Creates a copy of a file and applies any requested updates with patch semantics. Folders cannot be copied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**enforce_single_parent** | Option<**bool**> | Deprecated. Copying files into multiple folders is no longer supported. Use shortcuts instead. |  |
**ignore_default_visibility** | Option<**bool**> | Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**keep_revision_forever** | Option<**bool**> | Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions. |  |
**ocr_language** | Option<**String**> | A language hint for OCR processing during image import (ISO 639-1 code). |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**file** | Option<[**File**](File.md)> |  |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_create

> crate::models::File drive_files_create(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, enforce_single_parent, ignore_default_visibility, include_permissions_for_view, keep_revision_forever, ocr_language, supports_all_drives, supports_team_drives, use_content_as_indexable_text, file)


Creates a new file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**enforce_single_parent** | Option<**bool**> | Deprecated. Creating files in multiple folders is no longer supported. |  |
**ignore_default_visibility** | Option<**bool**> | Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**keep_revision_forever** | Option<**bool**> | Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions. |  |
**ocr_language** | Option<**String**> | A language hint for OCR processing during image import (ISO 639-1 code). |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**use_content_as_indexable_text** | Option<**bool**> | Whether to use the uploaded content as indexable text. |  |
**file** | Option<[**File**](File.md)> |  |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_delete

> drive_files_delete(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, enforce_single_parent, supports_all_drives, supports_team_drives)


Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**enforce_single_parent** | Option<**bool**> | Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |

### Return type

 (empty response body)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_empty_trash

> drive_files_empty_trash(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, enforce_single_parent)


Permanently deletes all of the user's trashed files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**enforce_single_parent** | Option<**bool**> | Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root. |  |

### Return type

 (empty response body)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_export

> drive_files_export(file_id, mime_type, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip)


Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**mime_type** | **String** | The MIME type of the format requested for this export. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |

### Return type

 (empty response body)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_generate_ids

> crate::models::GeneratedIds drive_files_generate_ids(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, count, space, _type)


Generates a set of file IDs which can be provided in create or copy requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**count** | Option<**i32**> | The number of IDs to return. |  |
**space** | Option<**String**> | The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'. (Default: 'drive') |  |
**_type** | Option<**String**> | The type of items which the IDs can be used for. Supported values are 'files' and 'shortcuts'. Note that 'shortcuts' are only supported in the drive 'space'. (Default: 'files') |  |

### Return type

[**crate::models::GeneratedIds**](GeneratedIds.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_get

> crate::models::File drive_files_get(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, acknowledge_abuse, include_permissions_for_view, supports_all_drives, supports_team_drives)


Gets a file's metadata or content by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**acknowledge_abuse** | Option<**bool**> | Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_list

> crate::models::FileList drive_files_list(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, corpora, corpus, drive_id, include_items_from_all_drives, include_permissions_for_view, include_team_drive_items, order_by, page_size, page_token, q, spaces, supports_all_drives, supports_team_drives, team_drive_id)


Lists or searches files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**corpora** | Option<**String**> | Groupings of files to which the query applies. Supported groupings are: 'user' (files created by, opened by, or shared directly with the user), 'drive' (files in the specified shared drive as indicated by the 'driveId'), 'domain' (files shared to the user's domain), and 'allDrives' (A combination of 'user' and 'drive' for all drives where the user is a member). When able, use 'user' or 'drive', instead of 'allDrives', for efficiency. |  |
**corpus** | Option<**String**> | The source of files to list. Deprecated: use 'corpora' instead. |  |
**drive_id** | Option<**String**> | ID of the shared drive to search. |  |
**include_items_from_all_drives** | Option<**bool**> | Whether both My Drive and shared drive items should be included in results. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**include_team_drive_items** | Option<**bool**> | Deprecated use includeItemsFromAllDrives instead. |  |
**order_by** | Option<**String**> | A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored. |  |
**page_size** | Option<**i32**> | The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached. |  |
**page_token** | Option<**String**> | The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response. |  |
**q** | Option<**String**> | A query for filtering the file results. See the \"Search for Files\" guide for supported syntax. |  |
**spaces** | Option<**String**> | A comma-separated list of spaces to query within the corpus. Supported values are 'drive' and 'appDataFolder'. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**team_drive_id** | Option<**String**> | Deprecated use driveId instead. |  |

### Return type

[**crate::models::FileList**](FileList.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_update

> crate::models::File drive_files_update(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, add_parents, enforce_single_parent, include_permissions_for_view, keep_revision_forever, ocr_language, remove_parents, supports_all_drives, supports_team_drives, use_content_as_indexable_text, file)


Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might change automatically, such as modifiedDate. This method supports patch semantics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**add_parents** | Option<**String**> | A comma-separated list of parent IDs to add. |  |
**enforce_single_parent** | Option<**bool**> | Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**keep_revision_forever** | Option<**bool**> | Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions. |  |
**ocr_language** | Option<**String**> | A language hint for OCR processing during image import (ISO 639-1 code). |  |
**remove_parents** | Option<**String**> | A comma-separated list of parent IDs to remove. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**use_content_as_indexable_text** | Option<**bool**> | Whether to use the uploaded content as indexable text. |  |
**file** | Option<[**File**](File.md)> |  |  |

### Return type

[**crate::models::File**](File.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_files_watch

> crate::models::Channel drive_files_watch(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, acknowledge_abuse, include_permissions_for_view, supports_all_drives, supports_team_drives, channel)


Subscribes to changes to a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**acknowledge_abuse** | Option<**bool**> | Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**channel** | Option<[**Channel**](Channel.md)> |  |  |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

