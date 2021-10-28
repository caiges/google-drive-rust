# \DrivesApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_drives_create**](DrivesApi.md#drive_drives_create) | **POST** /drives | 
[**drive_drives_delete**](DrivesApi.md#drive_drives_delete) | **DELETE** /drives/{driveId} | 
[**drive_drives_get**](DrivesApi.md#drive_drives_get) | **GET** /drives/{driveId} | 
[**drive_drives_hide**](DrivesApi.md#drive_drives_hide) | **POST** /drives/{driveId}/hide | 
[**drive_drives_list**](DrivesApi.md#drive_drives_list) | **GET** /drives | 
[**drive_drives_unhide**](DrivesApi.md#drive_drives_unhide) | **POST** /drives/{driveId}/unhide | 
[**drive_drives_update**](DrivesApi.md#drive_drives_update) | **PATCH** /drives/{driveId} | 



## drive_drives_create

> crate::models::Drive drive_drives_create(request_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, drive)


Creates a new shared drive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**drive** | Option<[**Drive**](Drive.md)> |  |  |

### Return type

[**crate::models::Drive**](Drive.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_drives_delete

> drive_drives_delete(drive_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip)


Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**drive_id** | **String** | The ID of the shared drive. | [required] |
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


## drive_drives_get

> crate::models::Drive drive_drives_get(drive_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, use_domain_admin_access)


Gets a shared drive's metadata by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**drive_id** | **String** | The ID of the shared drive. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs. |  |

### Return type

[**crate::models::Drive**](Drive.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_drives_hide

> crate::models::Drive drive_drives_hide(drive_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip)


Hides a shared drive from the default view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**drive_id** | **String** | The ID of the shared drive. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |

### Return type

[**crate::models::Drive**](Drive.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_drives_list

> crate::models::DriveList drive_drives_list(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, page_size, page_token, q, use_domain_admin_access)


Lists the user's shared drives.

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
**page_size** | Option<**i32**> | Maximum number of shared drives to return per page. |  |
**page_token** | Option<**String**> | Page token for shared drives. |  |
**q** | Option<**String**> | Query string for searching shared drives. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned. |  |

### Return type

[**crate::models::DriveList**](DriveList.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_drives_unhide

> crate::models::Drive drive_drives_unhide(drive_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip)


Restores a shared drive to the default view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**drive_id** | **String** | The ID of the shared drive. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |

### Return type

[**crate::models::Drive**](Drive.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_drives_update

> crate::models::Drive drive_drives_update(drive_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, use_domain_admin_access, drive)


Updates the metadate for a shared drive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**drive_id** | **String** | The ID of the shared drive. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**use_domain_admin_access** | Option<**bool**> | Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs. |  |
**drive** | Option<[**Drive**](Drive.md)> |  |  |

### Return type

[**crate::models::Drive**](Drive.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

