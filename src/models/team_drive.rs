/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TeamDrive : Deprecated: use the drive collection instead.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TeamDrive {
    #[serde(rename = "backgroundImageFile", skip_serializing_if = "Option::is_none")]
    pub background_image_file: Option<Box<crate::models::TeamDriveBackgroundImageFile>>,
    /// A short-lived link to this Team Drive's background image.
    #[serde(rename = "backgroundImageLink", skip_serializing_if = "Option::is_none")]
    pub background_image_link: Option<String>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<crate::models::TeamDriveCapabilities>>,
    /// The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId.
    #[serde(rename = "colorRgb", skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<String>,
    /// The time at which the Team Drive was created (RFC 3339 date-time).
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// The ID of this Team Drive which is also the ID of the top level folder of this Team Drive.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#teamDrive\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name of this Team Drive.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<crate::models::TeamDriveRestrictions>>,
    /// The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename = "themeId", skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

impl TeamDrive {
    /// Deprecated: use the drive collection instead.
    pub fn new() -> TeamDrive {
        TeamDrive {
            background_image_file: None,
            background_image_link: None,
            capabilities: None,
            color_rgb: None,
            created_time: None,
            id: None,
            kind: None,
            name: None,
            restrictions: None,
            theme_id: None,
        }
    }
}

