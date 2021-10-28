/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DriveCapabilities : Capabilities the current user has on this shared drive.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DriveCapabilities {
    /// Whether the current user can add children to folders in this shared drive.
    #[serde(rename = "canAddChildren", skip_serializing_if = "Option::is_none")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive.
    #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction", skip_serializing_if = "Option::is_none")]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this shared drive.
    #[serde(rename = "canChangeDomainUsersOnlyRestriction", skip_serializing_if = "Option::is_none")]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this shared drive.
    #[serde(rename = "canChangeDriveBackground", skip_serializing_if = "Option::is_none")]
    pub can_change_drive_background: Option<bool>,
    /// Whether the current user can change the driveMembersOnly restriction of this shared drive.
    #[serde(rename = "canChangeDriveMembersOnlyRestriction", skip_serializing_if = "Option::is_none")]
    pub can_change_drive_members_only_restriction: Option<bool>,
    /// Whether the current user can comment on files in this shared drive.
    #[serde(rename = "canComment", skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    /// Whether the current user can copy files in this shared drive.
    #[serde(rename = "canCopy", skip_serializing_if = "Option::is_none")]
    pub can_copy: Option<bool>,
    /// Whether the current user can delete children from folders in this shared drive.
    #[serde(rename = "canDeleteChildren", skip_serializing_if = "Option::is_none")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive.
    #[serde(rename = "canDeleteDrive", skip_serializing_if = "Option::is_none")]
    pub can_delete_drive: Option<bool>,
    /// Whether the current user can download files in this shared drive.
    #[serde(rename = "canDownload", skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
    /// Whether the current user can edit files in this shared drive
    #[serde(rename = "canEdit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of folders in this shared drive.
    #[serde(rename = "canListChildren", skip_serializing_if = "Option::is_none")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can add members to this shared drive or remove them or change their role.
    #[serde(rename = "canManageMembers", skip_serializing_if = "Option::is_none")]
    pub can_manage_members: Option<bool>,
    /// Whether the current user can read the revisions resource of files in this shared drive.
    #[serde(rename = "canReadRevisions", skip_serializing_if = "Option::is_none")]
    pub can_read_revisions: Option<bool>,
    /// Whether the current user can rename files or folders in this shared drive.
    #[serde(rename = "canRename", skip_serializing_if = "Option::is_none")]
    pub can_rename: Option<bool>,
    /// Whether the current user can rename this shared drive.
    #[serde(rename = "canRenameDrive", skip_serializing_if = "Option::is_none")]
    pub can_rename_drive: Option<bool>,
    /// Whether the current user can share files or folders in this shared drive.
    #[serde(rename = "canShare", skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// Whether the current user can trash children from folders in this shared drive.
    #[serde(rename = "canTrashChildren", skip_serializing_if = "Option::is_none")]
    pub can_trash_children: Option<bool>,
}

impl DriveCapabilities {
    /// Capabilities the current user has on this shared drive.
    pub fn new() -> DriveCapabilities {
        DriveCapabilities {
            can_add_children: None,
            can_change_copy_requires_writer_permission_restriction: None,
            can_change_domain_users_only_restriction: None,
            can_change_drive_background: None,
            can_change_drive_members_only_restriction: None,
            can_comment: None,
            can_copy: None,
            can_delete_children: None,
            can_delete_drive: None,
            can_download: None,
            can_edit: None,
            can_list_children: None,
            can_manage_members: None,
            can_read_revisions: None,
            can_rename: None,
            can_rename_drive: None,
            can_share: None,
            can_trash_children: None,
        }
    }
}


