# Drive

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**background_image_file** | Option<[**crate::models::DriveBackgroundImageFile**](Drive_backgroundImageFile.md)> |  | [optional]
**background_image_link** | Option<**String**> | A short-lived link to this shared drive's background image. | [optional]
**capabilities** | Option<[**crate::models::DriveCapabilities**](Drive_capabilities.md)> |  | [optional]
**color_rgb** | Option<**String**> | The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId. | [optional]
**created_time** | Option<**String**> | The time at which the shared drive was created (RFC 3339 date-time). | [optional]
**hidden** | Option<**bool**> | Whether the shared drive is hidden from default view. | [optional]
**id** | Option<**String**> | The ID of this shared drive which is also the ID of the top level folder of this shared drive. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#drive\". | [optional][default to drive#drive]
**name** | Option<**String**> | The name of this shared drive. | [optional]
**restrictions** | Option<[**crate::models::DriveRestrictions**](Drive_restrictions.md)> |  | [optional]
**theme_id** | Option<**String**> | The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


