# Revision

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**export_links** | Option<**::std::collections::HashMap<String, String>**> | Links for exporting Docs Editors files to specific formats. | [optional]
**id** | Option<**String**> | The ID of the revision. | [optional]
**keep_forever** | Option<**bool**> | Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#revision\". | [optional][default to drive#revision]
**last_modifying_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**md5_checksum** | Option<**String**> | The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive. | [optional]
**mime_type** | Option<**String**> | The MIME type of the revision. | [optional]
**modified_time** | Option<**String**> | The last time the revision was modified (RFC 3339 date-time). | [optional]
**original_filename** | Option<**String**> | The original filename used to create this revision. This is only applicable to files with binary content in Drive. | [optional]
**publish_auto** | Option<**bool**> | Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files. | [optional]
**published** | Option<**bool**> | Whether this revision is published. This is only applicable to Docs Editors files. | [optional]
**published_link** | Option<**String**> | A link to the published revision. This is only populated for Google Sites files. | [optional]
**published_outside_domain** | Option<**bool**> | Whether this revision is published outside the domain. This is only applicable to Docs Editors files. | [optional]
**size** | Option<**String**> | The size of the revision's content in bytes. This is only applicable to files with binary content in Drive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


