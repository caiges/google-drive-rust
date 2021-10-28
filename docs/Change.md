# Change

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_type** | Option<**String**> | The type of the change. Possible values are file and drive. | [optional]
**drive** | Option<[**crate::models::Drive**](Drive.md)> |  | [optional]
**drive_id** | Option<**String**> | The ID of the shared drive associated with this change. | [optional]
**file** | Option<[**crate::models::File**](File.md)> |  | [optional]
**file_id** | Option<**String**> | The ID of the file which has changed. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#change\". | [optional][default to drive#change]
**removed** | Option<**bool**> | Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access. | [optional]
**team_drive** | Option<[**crate::models::TeamDrive**](TeamDrive.md)> |  | [optional]
**team_drive_id** | Option<**String**> | Deprecated - use driveId instead. | [optional]
**time** | Option<**String**> | The time of this change (RFC 3339 date-time). | [optional]
**_type** | Option<**String**> | Deprecated - use changeType instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


