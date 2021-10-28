# ContentRestriction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**read_only** | Option<**bool**> | Whether the content of the file is read-only. If a file is read-only, a new revision of the file may not be added, comments may not be added or modified, and the title of the file may not be modified. | [optional]
**reason** | Option<**String**> | Reason for why the content of the file is restricted. This is only mutable on requests that also set readOnly=true. | [optional]
**restricting_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**restriction_time** | Option<**String**> | The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true. | [optional]
**_type** | Option<**String**> | The type of the content restriction. Currently the only possible value is globalContentRestriction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


