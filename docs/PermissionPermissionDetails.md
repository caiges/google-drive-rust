# PermissionPermissionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inherited** | Option<**bool**> | Whether this permission is inherited. This field is always populated. This is an output-only field. | [optional]
**inherited_from** | Option<**String**> | The ID of the item from which this permission is inherited. This is an output-only field. | [optional]
**permission_type** | Option<**String**> | The permission type for this user. While new values may be added in future, the following are currently possible:   - file  - member | [optional]
**role** | Option<**String**> | The primary role for this user. While new values may be added in the future, the following are currently possible:   - organizer  - fileOrganizer  - writer  - commenter  - reader | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


