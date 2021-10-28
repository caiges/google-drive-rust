# ChangeList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**changes** | Option<[**Vec<crate::models::Change>**](Change.md)> | The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#changeList\". | [optional][default to drive#changeList]
**new_start_page_token** | Option<**String**> | The starting page token for future changes. This will be present only if the end of the current changes list has been reached. | [optional]
**next_page_token** | Option<**String**> | The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


