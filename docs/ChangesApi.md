# \ChangesApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_changes_get_start_page_token**](ChangesApi.md#drive_changes_get_start_page_token) | **GET** /changes/startPageToken | 
[**drive_changes_list**](ChangesApi.md#drive_changes_list) | **GET** /changes | 
[**drive_changes_watch**](ChangesApi.md#drive_changes_watch) | **POST** /changes/watch | 



## drive_changes_get_start_page_token

> crate::models::StartPageToken drive_changes_get_start_page_token(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, drive_id, supports_all_drives, supports_team_drives, team_drive_id)


Gets the starting pageToken for listing future changes.

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
**drive_id** | Option<**String**> | The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive is returned. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**team_drive_id** | Option<**String**> | Deprecated use driveId instead. |  |

### Return type

[**crate::models::StartPageToken**](StartPageToken.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_changes_list

> crate::models::ChangeList drive_changes_list(page_token, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, drive_id, include_corpus_removals, include_items_from_all_drives, include_permissions_for_view, include_removed, include_team_drive_items, page_size, restrict_to_my_drive, spaces, supports_all_drives, supports_team_drives, team_drive_id)


Lists the changes for a user or shared drive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_token** | **String** | The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**drive_id** | Option<**String**> | The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier. |  |
**include_corpus_removals** | Option<**bool**> | Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file. |  |
**include_items_from_all_drives** | Option<**bool**> | Whether both My Drive and shared drive items should be included in results. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**include_removed** | Option<**bool**> | Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access. |  |
**include_team_drive_items** | Option<**bool**> | Deprecated use includeItemsFromAllDrives instead. |  |
**page_size** | Option<**i32**> | The maximum number of changes to return per page. |  |
**restrict_to_my_drive** | Option<**bool**> | Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive. |  |
**spaces** | Option<**String**> | A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**team_drive_id** | Option<**String**> | Deprecated use driveId instead. |  |

### Return type

[**crate::models::ChangeList**](ChangeList.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_changes_watch

> crate::models::Channel drive_changes_watch(page_token, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, drive_id, include_corpus_removals, include_items_from_all_drives, include_permissions_for_view, include_removed, include_team_drive_items, page_size, restrict_to_my_drive, spaces, supports_all_drives, supports_team_drives, team_drive_id, channel)


Subscribes to changes for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_token** | **String** | The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**drive_id** | Option<**String**> | The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier. |  |
**include_corpus_removals** | Option<**bool**> | Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file. |  |
**include_items_from_all_drives** | Option<**bool**> | Whether both My Drive and shared drive items should be included in results. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**include_removed** | Option<**bool**> | Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access. |  |
**include_team_drive_items** | Option<**bool**> | Deprecated use includeItemsFromAllDrives instead. |  |
**page_size** | Option<**i32**> | The maximum number of changes to return per page. |  |
**restrict_to_my_drive** | Option<**bool**> | Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive. |  |
**spaces** | Option<**String**> | A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**team_drive_id** | Option<**String**> | Deprecated use driveId instead. |  |
**channel** | Option<[**Channel**](Channel.md)> |  |  |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

