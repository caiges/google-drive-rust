# PermissionList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#permissionList\". | [optional][default to drive#permissionList]
**next_page_token** | Option<**String**> | The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]
**permissions** | Option<[**Vec<crate::models::Permission>**](Permission.md)> | The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


