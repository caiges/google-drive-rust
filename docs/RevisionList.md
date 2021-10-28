# RevisionList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#revisionList\". | [optional][default to drive#revisionList]
**next_page_token** | Option<**String**> | The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]
**revisions** | Option<[**Vec<crate::models::Revision>**](Revision.md)> | The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


