/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Change : A change to a file or shared drive.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Change {
    /// The type of the change. Possible values are file and drive.
    #[serde(rename = "changeType", skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(rename = "drive", skip_serializing_if = "Option::is_none")]
    pub drive: Option<Box<crate::models::Drive>>,
    /// The ID of the shared drive associated with this change.
    #[serde(rename = "driveId", skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<crate::models::File>>,
    /// The ID of the file which has changed.
    #[serde(rename = "fileId", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#change\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access.
    #[serde(rename = "removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
    #[serde(rename = "teamDrive", skip_serializing_if = "Option::is_none")]
    pub team_drive: Option<Box<crate::models::TeamDrive>>,
    /// Deprecated - use driveId instead.
    #[serde(rename = "teamDriveId", skip_serializing_if = "Option::is_none")]
    pub team_drive_id: Option<String>,
    /// The time of this change (RFC 3339 date-time).
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Deprecated - use changeType instead.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl Change {
    /// A change to a file or shared drive.
    pub fn new() -> Change {
        Change {
            change_type: None,
            drive: None,
            drive_id: None,
            file: None,
            file_id: None,
            kind: None,
            removed: None,
            team_drive: None,
            team_drive_id: None,
            time: None,
            _type: None,
        }
    }
}


