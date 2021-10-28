# Permission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_file_discovery** | Option<**bool**> | Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone. | [optional]
**deleted** | Option<**bool**> | Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions. | [optional]
**display_name** | Option<**String**> | The \"pretty\" name of the value of the permission. The following is a list of examples for each type of permission:   - user - User's full name, as defined for their Google account, such as \"Joe Smith.\"  - group - Name of the Google Group, such as \"The Company Administrators.\"  - domain - String domain name, such as \"thecompany.com.\"  - anyone - No displayName is present. | [optional]
**domain** | Option<**String**> | The domain to which this permission refers. | [optional]
**email_address** | Option<**String**> | The email address of the user or group to which this permission refers. | [optional]
**expiration_time** | Option<**String**> | The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:   - They can only be set on user and group permissions  - The time must be in the future  - The time cannot be more than a year in the future | [optional]
**id** | Option<**String**> | The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId. IDs should be treated as opaque values. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#permission\". | [optional][default to drive#permission]
**permission_details** | Option<[**Vec<crate::models::PermissionPermissionDetails>**](Permission_permissionDetails.md)> | Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items. | [optional][readonly]
**photo_link** | Option<**String**> | A link to the user's profile photo, if available. | [optional]
**role** | Option<**String**> | The role granted by this permission. While new values may be supported in the future, the following are currently allowed:   - owner  - organizer  - fileOrganizer  - writer  - commenter  - reader | [optional]
**team_drive_permission_details** | Option<[**Vec<crate::models::PermissionTeamDrivePermissionDetails>**](Permission_teamDrivePermissionDetails.md)> | Deprecated - use permissionDetails instead. | [optional][readonly]
**_type** | Option<**String**> | The type of the grantee. Valid values are:   - user  - group  - domain  - anyone  When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type. | [optional]
**view** | Option<**String**> | Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


