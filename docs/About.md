# About

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_installed** | Option<**bool**> | Whether the user has installed the requesting app. | [optional]
**can_create_drives** | Option<**bool**> | Whether the user can create shared drives. | [optional]
**can_create_team_drives** | Option<**bool**> | Deprecated - use canCreateDrives instead. | [optional]
**drive_themes** | Option<[**Vec<crate::models::AboutDriveThemes>**](About_driveThemes.md)> | A list of themes that are supported for shared drives. | [optional]
**export_formats** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | A map of source MIME type to possible targets for all supported exports. | [optional]
**folder_color_palette** | Option<**Vec<String>**> | The currently supported folder colors as RGB hex strings. | [optional]
**import_formats** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | A map of source MIME type to possible targets for all supported imports. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#about\". | [optional][default to drive#about]
**max_import_sizes** | Option<**::std::collections::HashMap<String, String>**> | A map of maximum import sizes by MIME type, in bytes. | [optional]
**max_upload_size** | Option<**String**> | The maximum upload size in bytes. | [optional]
**storage_quota** | Option<[**crate::models::AboutStorageQuota**](About_storageQuota.md)> |  | [optional]
**team_drive_themes** | Option<[**Vec<crate::models::AboutTeamDriveThemes>**](About_teamDriveThemes.md)> | Deprecated - use driveThemes instead. | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


