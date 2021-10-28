/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Reply : A reply to a comment on a file.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Reply {
    /// The action the reply performed to the parent comment. Valid values are:   - resolve  - reopen
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<crate::models::User>>,
    /// The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The time at which the reply was created (RFC 3339 date-time).
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// Whether the reply has been deleted. A deleted reply has no content.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The content of the reply with HTML formatting.
    #[serde(rename = "htmlContent", skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// The ID of the reply.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#reply\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The last time the reply was modified (RFC 3339 date-time).
    #[serde(rename = "modifiedTime", skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
}

impl Reply {
    /// A reply to a comment on a file.
    pub fn new() -> Reply {
        Reply {
            action: None,
            author: None,
            content: None,
            created_time: None,
            deleted: None,
            html_content: None,
            id: None,
            kind: None,
            modified_time: None,
        }
    }
}


