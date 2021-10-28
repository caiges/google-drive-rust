/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FileContentHintsThumbnail : A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileContentHintsThumbnail {
    /// The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5).
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The MIME type of the thumbnail.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl FileContentHintsThumbnail {
    /// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
    pub fn new() -> FileContentHintsThumbnail {
        FileContentHintsThumbnail {
            image: None,
            mime_type: None,
        }
    }
}

