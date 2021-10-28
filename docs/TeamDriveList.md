# TeamDriveList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#teamDriveList\". | [optional][default to drive#teamDriveList]
**next_page_token** | Option<**String**> | The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results. | [optional]
**team_drives** | Option<[**Vec<crate::models::TeamDrive>**](TeamDrive.md)> | The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


