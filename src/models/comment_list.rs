/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommentList : A list of comments on a file.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommentList {
    /// The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::models::Comment>>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#commentList\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl CommentList {
    /// A list of comments on a file.
    pub fn new() -> CommentList {
        CommentList {
            comments: None,
            kind: None,
            next_page_token: None,
        }
    }
}

