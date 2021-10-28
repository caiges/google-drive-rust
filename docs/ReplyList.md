# ReplyList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#replyList\". | [optional][default to drive#replyList]
**next_page_token** | Option<**String**> | The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]
**replies** | Option<[**Vec<crate::models::Reply>**](Reply.md)> | The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


