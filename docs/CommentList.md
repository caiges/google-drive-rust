# CommentList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comments** | Option<[**Vec<crate::models::Comment>**](Comment.md)> | The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#commentList\". | [optional][default to drive#commentList]
**next_page_token** | Option<**String**> | The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


