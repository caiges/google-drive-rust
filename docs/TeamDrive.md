# TeamDrive

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**background_image_file** | Option<[**crate::models::TeamDriveBackgroundImageFile**](TeamDrive_backgroundImageFile.md)> |  | [optional]
**background_image_link** | Option<**String**> | A short-lived link to this Team Drive's background image. | [optional]
**capabilities** | Option<[**crate::models::TeamDriveCapabilities**](TeamDrive_capabilities.md)> |  | [optional]
**color_rgb** | Option<**String**> | The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId. | [optional]
**created_time** | Option<**String**> | The time at which the Team Drive was created (RFC 3339 date-time). | [optional]
**id** | Option<**String**> | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#teamDrive\". | [optional][default to drive#teamDrive]
**name** | Option<**String**> | The name of this Team Drive. | [optional]
**restrictions** | Option<[**crate::models::TeamDriveRestrictions**](TeamDrive_restrictions.md)> |  | [optional]
**theme_id** | Option<**String**> | The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


