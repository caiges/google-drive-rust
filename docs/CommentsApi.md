# \CommentsApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_comments_create**](CommentsApi.md#drive_comments_create) | **POST** /files/{fileId}/comments | 
[**drive_comments_delete**](CommentsApi.md#drive_comments_delete) | **DELETE** /files/{fileId}/comments/{commentId} | 
[**drive_comments_get**](CommentsApi.md#drive_comments_get) | **GET** /files/{fileId}/comments/{commentId} | 
[**drive_comments_list**](CommentsApi.md#drive_comments_list) | **GET** /files/{fileId}/comments | 
[**drive_comments_update**](CommentsApi.md#drive_comments_update) | **PATCH** /files/{fileId}/comments/{commentId} | 



## drive_comments_create

> crate::models::Comment drive_comments_create(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, comment)


Creates a new comment on a file.

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
**comment** | Option<[**Comment**](Comment.md)> |  |  |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_comments_delete

> drive_comments_delete(file_id, comment_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip)


Deletes a comment.

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

### Return type

 (empty response body)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_comments_get

> crate::models::Comment drive_comments_get(file_id, comment_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, include_deleted)


Gets a comment by ID.

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
**include_deleted** | Option<**bool**> | Whether to return deleted comments. Deleted comments will not include their original content. |  |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_comments_list

> crate::models::CommentList drive_comments_list(file_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, include_deleted, page_size, page_token, start_modified_time)


Lists a file's comments.

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
**include_deleted** | Option<**bool**> | Whether to include deleted comments. Deleted comments will not include their original content. |  |
**page_size** | Option<**i32**> | The maximum number of comments to return per page. |  |
**page_token** | Option<**String**> | The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response. |  |
**start_modified_time** | Option<**String**> | The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time). |  |

### Return type

[**crate::models::CommentList**](CommentList.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drive_comments_update

> crate::models::Comment drive_comments_update(file_id, comment_id, alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, comment)


Updates a comment with patch semantics.

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
**comment** | Option<[**Comment**](Comment.md)> |  |  |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

