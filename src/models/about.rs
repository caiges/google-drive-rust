/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// About : Information about the user, the user's Drive, and system capabilities.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct About {
    /// Whether the user has installed the requesting app.
    #[serde(rename = "appInstalled", skip_serializing_if = "Option::is_none")]
    pub app_installed: Option<bool>,
    /// Whether the user can create shared drives.
    #[serde(rename = "canCreateDrives", skip_serializing_if = "Option::is_none")]
    pub can_create_drives: Option<bool>,
    /// Deprecated - use canCreateDrives instead.
    #[serde(rename = "canCreateTeamDrives", skip_serializing_if = "Option::is_none")]
    pub can_create_team_drives: Option<bool>,
    /// A list of themes that are supported for shared drives.
    #[serde(rename = "driveThemes", skip_serializing_if = "Option::is_none")]
    pub drive_themes: Option<Vec<crate::models::AboutDriveThemes>>,
    /// A map of source MIME type to possible targets for all supported exports.
    #[serde(rename = "exportFormats", skip_serializing_if = "Option::is_none")]
    pub export_formats: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// The currently supported folder colors as RGB hex strings.
    #[serde(rename = "folderColorPalette", skip_serializing_if = "Option::is_none")]
    pub folder_color_palette: Option<Vec<String>>,
    /// A map of source MIME type to possible targets for all supported imports.
    #[serde(rename = "importFormats", skip_serializing_if = "Option::is_none")]
    pub import_formats: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#about\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A map of maximum import sizes by MIME type, in bytes.
    #[serde(rename = "maxImportSizes", skip_serializing_if = "Option::is_none")]
    pub max_import_sizes: Option<::std::collections::HashMap<String, String>>,
    /// The maximum upload size in bytes.
    #[serde(rename = "maxUploadSize", skip_serializing_if = "Option::is_none")]
    pub max_upload_size: Option<String>,
    #[serde(rename = "storageQuota", skip_serializing_if = "Option::is_none")]
    pub storage_quota: Option<Box<crate::models::AboutStorageQuota>>,
    /// Deprecated - use driveThemes instead.
    #[serde(rename = "teamDriveThemes", skip_serializing_if = "Option::is_none")]
    pub team_drive_themes: Option<Vec<crate::models::AboutTeamDriveThemes>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl About {
    /// Information about the user, the user's Drive, and system capabilities.
    pub fn new() -> About {
        About {
            app_installed: None,
            can_create_drives: None,
            can_create_team_drives: None,
            drive_themes: None,
            export_formats: None,
            folder_color_palette: None,
            import_formats: None,
            kind: None,
            max_import_sizes: None,
            max_upload_size: None,
            storage_quota: None,
            team_drive_themes: None,
            user: None,
        }
    }
}

