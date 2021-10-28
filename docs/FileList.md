# FileList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**files** | Option<[**Vec<crate::models::File>**](File.md)> | The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]
**incomplete_search** | Option<**bool**> | Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the \"allDrives\" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as \"user\" or \"drive\". | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#fileList\". | [optional][default to drive#fileList]
**next_page_token** | Option<**String**> | The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


