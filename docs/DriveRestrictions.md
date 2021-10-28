# DriveRestrictions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_managed_restrictions** | Option<**bool**> | Whether administrative privileges on this shared drive are required to modify restrictions. | [optional]
**copy_requires_writer_permission** | Option<**bool**> | Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive. | [optional]
**domain_users_only** | Option<**bool**> | Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive. | [optional]
**drive_members_only** | Option<**bool**> | Whether access to items inside this shared drive is restricted to its members. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


