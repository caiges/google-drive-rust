# DriveList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**drives** | Option<[**Vec<crate::models::Drive>**](Drive.md)> | The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#driveList\". | [optional][default to drive#driveList]
**next_page_token** | Option<**String**> | The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


