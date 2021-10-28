# TeamDriveRestrictions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_managed_restrictions** | Option<**bool**> | Whether administrative privileges on this Team Drive are required to modify restrictions. | [optional]
**copy_requires_writer_permission** | Option<**bool**> | Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive. | [optional]
**domain_users_only** | Option<**bool**> | Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive. | [optional]
**team_members_only** | Option<**bool**> | Whether access to items inside this Team Drive is restricted to members of this Team Drive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


