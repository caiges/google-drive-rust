# \ChannelsApi

All URIs are relative to *https://www.googleapis.com/drive/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**drive_channels_stop**](ChannelsApi.md#drive_channels_stop) | **POST** /channels/stop | 



## drive_channels_stop

> drive_channels_stop(alt, fields, key, oauth_token, pretty_print, quota_user, user_ip, channel)


Stop watching resources through this channel

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
**channel** | Option<[**Channel**](Channel.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Oauth2](../README.md#Oauth2), [Oauth2c](../README.md#Oauth2c)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

