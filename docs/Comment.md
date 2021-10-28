# Comment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anchor** | Option<**String**> | A region of the document represented as a JSON string. For details on defining anchor properties, refer to  Add comments and replies. | [optional]
**author** | Option<[**crate::models::User**](User.md)> |  | [optional]
**content** | Option<**String**> | The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed. | [optional]
**created_time** | Option<**String**> | The time at which the comment was created (RFC 3339 date-time). | [optional]
**deleted** | Option<**bool**> | Whether the comment has been deleted. A deleted comment has no content. | [optional]
**html_content** | Option<**String**> | The content of the comment with HTML formatting. | [optional]
**id** | Option<**String**> | The ID of the comment. | [optional]
**kind** | Option<**String**> | Identifies what kind of resource this is. Value: the fixed string \"drive#comment\". | [optional][default to drive#comment]
**modified_time** | Option<**String**> | The last time the comment or any of its replies was modified (RFC 3339 date-time). | [optional]
**quoted_file_content** | Option<[**crate::models::CommentQuotedFileContent**](Comment_quotedFileContent.md)> |  | [optional]
**replies** | Option<[**Vec<crate::models::Reply>**](Reply.md)> | The full list of replies to the comment in chronological order. | [optional]
**resolved** | Option<**bool**> | Whether the comment has been resolved by one of its replies. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


