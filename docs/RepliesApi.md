# \RepliesApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_replies_create**](RepliesApi.md#drive_replies_create) | **POST** /files/{fileId}/comments/{commentId}/replies | 
[**drive_replies_delete**](RepliesApi.md#drive_replies_delete) | **DELETE** /files/{fileId}/comments/{commentId}/replies/{replyId} | 
[**drive_replies_get**](RepliesApi.md#drive_replies_get) | **GET** /files/{fileId}/comments/{commentId}/replies/{replyId} | 
[**drive_replies_list**](RepliesApi.md#drive_replies_list) | **GET** /files/{fileId}/comments/{commentId}/replies | 
[**drive_replies_update**](RepliesApi.md#drive_replies_update) | **PATCH** /files/{fileId}/comments/{commentId}/replies/{replyId} | 



## drive_replies_create

> crate::models::Reply drive_replies_create(file_id, comment_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, reply)


Creates a new reply to a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**comment_id** | **String** | The ID of the comment. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**reply** | Option<[**Reply**](Reply.md)> |  |  |

### Return type

[**crate::models::Reply**](Reply.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_replies_delete

> drive_replies_delete(file_id, comment_id, reply_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip)


Deletes a reply.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**comment_id** | **String** | The ID of the comment. | [required] |
**reply_id** | **String** | The ID of the reply. | [required] |
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


## drive_replies_get

> crate::models::Reply drive_replies_get(file_id, comment_id, reply_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, include_deleted)


Gets a reply by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**comment_id** | **String** | The ID of the comment. | [required] |
**reply_id** | **String** | The ID of the reply. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**include_deleted** | Option<**bool**> | Whether to return deleted replies. Deleted replies will not include their original content. |  |

### Return type

[**crate::models::Reply**](Reply.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_replies_list

> crate::models::ReplyList drive_replies_list(file_id, comment_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, include_deleted, page_size, page_token)


Lists a comment's replies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**comment_id** | **String** | The ID of the comment. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**include_deleted** | Option<**bool**> | Whether to include deleted replies. Deleted replies will not include their original content. |  |
**page_size** | Option<**i32**> | The maximum number of replies to return per page. |  |
**page_token** | Option<**String**> | The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response. |  |

### Return type

[**crate::models::ReplyList**](ReplyList.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_replies_update

> crate::models::Reply drive_replies_update(file_id, comment_id, reply_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, reply)


Updates a reply with patch semantics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file. | [required] |
**comment_id** | **String** | The ID of the comment. | [required] |
**reply_id** | **String** | The ID of the reply. | [required] |
**alt** | Option<**String**> | Data format for the response. |  |
**fields** | Option<**String**> | Selector specifying which fields to include in a partial response. |  |
**key** | Option<**String**> | API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token. |  |
**oauth_token** | Option<**String**> | OAuth 2.0 token for the current user. |  |
**pretty_print** | Option<**bool**> | Returns response with indentations and line breaks. |  |
**quota_user** | Option<**String**> | An opaque string that represents a user for quota purposes. Must not exceed 40 characters. |  |
**user_ip** | Option<**String**> | Deprecated. Please use quotaUser instead. |  |
**reply** | Option<[**Reply**](Reply.md)> |  |  |

### Return type

[**crate::models::Reply**](Reply.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

