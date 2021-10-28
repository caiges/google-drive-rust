# \PermissionsApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_permissions_create**](PermissionsApi.md#drive_permissions_create) | **POST** /files/{fileId}/permissions | 
[**drive_permissions_delete**](PermissionsApi.md#drive_permissions_delete) | **DELETE** /files/{fileId}/permissions/{permissionId} | 
[**drive_permissions_get**](PermissionsApi.md#drive_permissions_get) | **GET** /files/{fileId}/permissions/{permissionId} | 
[**drive_permissions_list**](PermissionsApi.md#drive_permissions_list) | **GET** /files/{fileId}/permissions | 
[**drive_permissions_update**](PermissionsApi.md#drive_permissions_update) | **PATCH** /files/{fileId}/permissions/{permissionId} | 



## drive_permissions_create

> crate::models::Permission drive_permissions_create(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, email_message, enforce_single_parent, move_to_new_owners_root, send_notification_email, supports_all_drives, supports_team_drives, transfer_ownership, use_domain_admin_access, permission)


Creates a permission for a file or shared drive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file or shared drive. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**email_message** | Option<**String**> | A plain text custom message to include in the notification email. |  |
**enforce_single_parent** | Option<**bool**> | Deprecated. See moveToNewOwnersRoot for details. |  |
**move_to_new_owners_root** | Option<**bool**> | This parameter will only take effect if the item is not in a shared drive and the request is attempting to transfer the ownership of the item. If set to true, the item will be moved to the new owner's My Drive root folder and all prior parents removed. If set to false, parents are not changed. |  |
**send_notification_email** | Option<**bool**> | Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**transfer_ownership** | Option<**bool**> | Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. File owners can only transfer ownership of files existing on My Drive. Files existing in a shared drive are owned by the organization that owns that shared drive. Ownership transfers are not supported for files and folders in shared drives. Organizers of a shared drive can move items from that shared drive into their My Drive which transfers the ownership to them. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs. |  |
**permission** | Option<[**Permission**](Permission.md)> |  |  |

### Return type

[**crate::models::Permission**](Permission.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_permissions_delete

> drive_permissions_delete(file_id, permission_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, supports_all_drives, supports_team_drives, use_domain_admin_access)


Deletes a permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file or shared drive. | [required] |
**permission_id** | **String** | The ID of the permission. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs. |  |

### Return type

 (empty response body)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_permissions_get

> crate::models::Permission drive_permissions_get(file_id, permission_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, supports_all_drives, supports_team_drives, use_domain_admin_access)


Gets a permission by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**permission_id** | **String** | The ID of the permission. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs. |  |

### Return type

[**crate::models::Permission**](Permission.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_permissions_list

> crate::models::PermissionList drive_permissions_list(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, include_permissions_for_view, page_size, page_token, supports_all_drives, supports_team_drives, use_domain_admin_access)


Lists a file's or shared drive's permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file or shared drive. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**include_permissions_for_view** | Option<**String**> | Specifies which additional view's permissions to include in the response. Only 'published' is supported. |  |
**page_size** | Option<**i32**> | The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned. |  |
**page_token** | Option<**String**> | The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs. |  |

### Return type

[**crate::models::PermissionList**](PermissionList.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_permissions_update

> crate::models::Permission drive_permissions_update(file_id, permission_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, remove_expiration, supports_all_drives, supports_team_drives, transfer_ownership, use_domain_admin_access, permission)


Updates a permission with patch semantics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file or shared drive. | [required] |
**permission_id** | **String** | The ID of the permission. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**remove_expiration** | Option<**bool**> | Whether to remove the expiration date. |  |
**supports_all_drives** | Option<**bool**> | Whether the requesting application supports both My Drives and shared drives. |  |
**supports_team_drives** | Option<**bool**> | Deprecated use supportsAllDrives instead. |  |
**transfer_ownership** | Option<**bool**> | Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. File owners can only transfer ownership of files existing on My Drive. Files existing in a shared drive are owned by the organization that owns that shared drive. Ownership transfers are not supported for files and folders in shared drives. Organizers of a shared drive can move items from that shared drive into their My Drive which transfers the ownership to them. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs. |  |
**permission** | Option<[**Permission**](Permission.md)> |  |  |

### Return type

[**crate::models::Permission**](Permission.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

