/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FileShortcutDetails : Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileShortcutDetails {
    /// The ID of the file that this shortcut points to.
    #[serde(rename = "targetId", skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// The MIME type of the file that this shortcut points to. The value of this field is a snapshot of the target's MIME type, captured when the shortcut is created.
    #[serde(rename = "targetMimeType", skip_serializing_if = "Option::is_none")]
    pub target_mime_type: Option<String>,
    /// The ResourceKey for the target file.
    #[serde(rename = "targetResourceKey", skip_serializing_if = "Option::is_none")]
    pub target_resource_key: Option<String>,
}

impl FileShortcutDetails {
    /// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
    pub fn new() -> FileShortcutDetails {
        FileShortcutDetails {
            target_id: None,
            target_mime_type: None,
            target_resource_key: None,
        }
    }
}

