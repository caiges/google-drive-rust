/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionTeamDrivePermissionDetails {
    /// Deprecated - use permissionDetails/inherited instead.
    #[serde(rename = "inherited", skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /// Deprecated - use permissionDetails/inheritedFrom instead.
    #[serde(rename = "inheritedFrom", skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    /// Deprecated - use permissionDetails/role instead.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Deprecated - use permissionDetails/permissionType instead.
    #[serde(rename = "teamDrivePermissionType", skip_serializing_if = "Option::is_none")]
    pub team_drive_permission_type: Option<String>,
}

impl PermissionTeamDrivePermissionDetails {
    pub fn new() -> PermissionTeamDrivePermissionDetails {
        PermissionTeamDrivePermissionDetails {
            inherited: None,
            inherited_from: None,
            role: None,
            team_drive_permission_type: None,
        }
    }
}


