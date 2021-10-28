# Reply

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | The action the reply performed to the parent comment. Valid values are:   - resolve  - reopen | [optional]
**author** | Option<[**crate::models::User**](User.md)> |  | [optional]
**content** | Option<**String**> | The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified. | [optional]
**created_time** | Option<**String**> | The time at which the reply was created (RFC 3339 date-time). | [optional]
**deleted** | Option<**bool**> | Whether the reply has been deleted. A deleted reply has no content. | [optional]
**html_content** | Option<**String**> | The content of the reply with HTML formatting. | [optional]
**id** | Option<**String**> | The ID of the reply. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#reply\". | [optional][default to drive#reply]
**modified_time** | Option<**String**> | The last time the reply was modified (RFC 3339 date-time). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


