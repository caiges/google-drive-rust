/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DriveList : A list of shared drives.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DriveList {
    /// The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "drives", skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<crate::models::Drive>>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#driveList\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl DriveList {
    /// A list of shared drives.
    pub fn new() -> DriveList {
        DriveList {
            drives: None,
            kind: None,
            next_page_token: None,
        }
    }
}


