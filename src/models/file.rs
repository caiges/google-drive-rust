/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// File : The metadata for a file.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct File {
    /// A collection of arbitrary key-value pairs which are private to the requesting app. Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties.
    #[serde(rename = "appProperties", skip_serializing_if = "Option::is_none")]
    pub app_properties: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<crate::models::FileCapabilities>>,
    #[serde(rename = "contentHints", skip_serializing_if = "Option::is_none")]
    pub content_hints: Option<Box<crate::models::FileContentHints>>,
    /// Restrictions for accessing the content of the file. Only populated if such a restriction exists.
    #[serde(rename = "contentRestrictions", skip_serializing_if = "Option::is_none")]
    pub content_restrictions: Option<Vec<crate::models::ContentRestriction>>,
    /// Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
    #[serde(rename = "copyRequiresWriterPermission", skip_serializing_if = "Option::is_none")]
    pub copy_requires_writer_permission: Option<bool>,
    /// The time at which the file was created (RFC 3339 date-time).
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// A short description of the file.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the shared drive the file resides in. Only populated for items in shared drives.
    #[serde(rename = "driveId", skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    /// Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder.
    #[serde(rename = "explicitlyTrashed", skip_serializing_if = "Option::is_none")]
    pub explicitly_trashed: Option<bool>,
    /// Links for exporting Docs Editors files to specific formats.
    #[serde(rename = "exportLinks", skip_serializing_if = "Option::is_none")]
    pub export_links: Option<::std::collections::HashMap<String, String>>,
    /// The final component of fullFileExtension. This is only available for files with binary content in Google Drive.
    #[serde(rename = "fileExtension", skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// The color for a folder or shortcut to a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource. If an unsupported color is specified, the closest color in the palette will be used instead.
    #[serde(rename = "folderColorRgb", skip_serializing_if = "Option::is_none")]
    pub folder_color_rgb: Option<String>,
    /// The full file extension extracted from the name field. May contain multiple concatenated extensions, such as \"tar.gz\". This is only available for files with binary content in Google Drive. This is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension.
    #[serde(rename = "fullFileExtension", skip_serializing_if = "Option::is_none")]
    pub full_file_extension: Option<String>,
    /// Whether there are permissions directly on this file. This field is only populated for items in shared drives.
    #[serde(rename = "hasAugmentedPermissions", skip_serializing_if = "Option::is_none")]
    pub has_augmented_permissions: Option<bool>,
    /// Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
    #[serde(rename = "hasThumbnail", skip_serializing_if = "Option::is_none")]
    pub has_thumbnail: Option<bool>,
    /// The ID of the file's head revision. This is currently only available for files with binary content in Google Drive.
    #[serde(rename = "headRevisionId", skip_serializing_if = "Option::is_none")]
    pub head_revision_id: Option<String>,
    /// A static, unauthenticated link to the file's icon.
    #[serde(rename = "iconLink", skip_serializing_if = "Option::is_none")]
    pub icon_link: Option<String>,
    /// The ID of the file.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageMediaMetadata", skip_serializing_if = "Option::is_none")]
    pub image_media_metadata: Option<Box<crate::models::FileImageMediaMetadata>>,
    /// Whether the file was created or opened by the requesting app.
    #[serde(rename = "isAppAuthorized", skip_serializing_if = "Option::is_none")]
    pub is_app_authorized: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#file\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "lastModifyingUser", skip_serializing_if = "Option::is_none")]
    pub last_modifying_user: Option<Box<crate::models::User>>,
    #[serde(rename = "linkShareMetadata", skip_serializing_if = "Option::is_none")]
    pub link_share_metadata: Option<Box<crate::models::FileLinkShareMetadata>>,
    /// The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive.
    #[serde(rename = "md5Checksum", skip_serializing_if = "Option::is_none")]
    pub md5_checksum: Option<String>,
    /// The MIME type of the file. Google Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Whether the file has been modified by this user.
    #[serde(rename = "modifiedByMe", skip_serializing_if = "Option::is_none")]
    pub modified_by_me: Option<bool>,
    /// The last time the file was modified by the user (RFC 3339 date-time).
    #[serde(rename = "modifiedByMeTime", skip_serializing_if = "Option::is_none")]
    pub modified_by_me_time: Option<String>,
    /// The last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user.
    #[serde(rename = "modifiedTime", skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive.
    #[serde(rename = "originalFilename", skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
    /// Whether the user owns the file. Not populated for items in shared drives.
    #[serde(rename = "ownedByMe", skip_serializing_if = "Option::is_none")]
    pub owned_by_me: Option<bool>,
    /// The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives.
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<crate::models::User>>,
    /// The IDs of the parent folders which contain the file. If not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list.
    #[serde(rename = "parents", skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    /// List of permission IDs for users with access to this file.
    #[serde(rename = "permissionIds", skip_serializing_if = "Option::is_none")]
    pub permission_ids: Option<Vec<String>>,
    /// The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::Permission>>,
    /// A collection of arbitrary key-value pairs which are visible to all apps. Entries with null values are cleared in update and copy requests.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
    /// The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled.
    #[serde(rename = "quotaBytesUsed", skip_serializing_if = "Option::is_none")]
    pub quota_bytes_used: Option<String>,
    /// A key needed to access the item via a shared link.
    #[serde(rename = "resourceKey", skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<String>,
    /// Whether the file has been shared. Not populated for items in shared drives.
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    /// The time at which the file was shared with the user, if applicable (RFC 3339 date-time).
    #[serde(rename = "sharedWithMeTime", skip_serializing_if = "Option::is_none")]
    pub shared_with_me_time: Option<String>,
    #[serde(rename = "sharingUser", skip_serializing_if = "Option::is_none")]
    pub sharing_user: Option<Box<crate::models::User>>,
    #[serde(rename = "shortcutDetails", skip_serializing_if = "Option::is_none")]
    pub shortcut_details: Option<Box<crate::models::FileShortcutDetails>>,
    /// The size of the file's content in bytes. This is applicable to binary files in Google Drive and Google Docs files.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'.
    #[serde(rename = "spaces", skip_serializing_if = "Option::is_none")]
    pub spaces: Option<Vec<String>>,
    /// Whether the user has starred the file.
    #[serde(rename = "starred", skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    /// Deprecated - use driveId instead.
    #[serde(rename = "teamDriveId", skip_serializing_if = "Option::is_none")]
    pub team_drive_id: Option<String>,
    /// A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request.
    #[serde(rename = "thumbnailLink", skip_serializing_if = "Option::is_none")]
    pub thumbnail_link: Option<String>,
    /// The thumbnail version for use in thumbnail cache invalidation.
    #[serde(rename = "thumbnailVersion", skip_serializing_if = "Option::is_none")]
    pub thumbnail_version: Option<String>,
    /// Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.
    #[serde(rename = "trashed", skip_serializing_if = "Option::is_none")]
    pub trashed: Option<bool>,
    /// The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.
    #[serde(rename = "trashedTime", skip_serializing_if = "Option::is_none")]
    pub trashed_time: Option<String>,
    #[serde(rename = "trashingUser", skip_serializing_if = "Option::is_none")]
    pub trashing_user: Option<Box<crate::models::User>>,
    /// A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "videoMediaMetadata", skip_serializing_if = "Option::is_none")]
    pub video_media_metadata: Option<Box<crate::models::FileVideoMediaMetadata>>,
    /// Whether the file has been viewed by this user.
    #[serde(rename = "viewedByMe", skip_serializing_if = "Option::is_none")]
    pub viewed_by_me: Option<bool>,
    /// The last time the file was viewed by the user (RFC 3339 date-time).
    #[serde(rename = "viewedByMeTime", skip_serializing_if = "Option::is_none")]
    pub viewed_by_me_time: Option<String>,
    /// Deprecated - use copyRequiresWriterPermission instead.
    #[serde(rename = "viewersCanCopyContent", skip_serializing_if = "Option::is_none")]
    pub viewers_can_copy_content: Option<bool>,
    /// A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive.
    #[serde(rename = "webContentLink", skip_serializing_if = "Option::is_none")]
    pub web_content_link: Option<String>,
    /// A link for opening the file in a relevant Google editor or viewer in a browser.
    #[serde(rename = "webViewLink", skip_serializing_if = "Option::is_none")]
    pub web_view_link: Option<String>,
    /// Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives.
    #[serde(rename = "writersCanShare", skip_serializing_if = "Option::is_none")]
    pub writers_can_share: Option<bool>,
}

impl File {
    /// The metadata for a file.
    pub fn new() -> File {
        File {
            app_properties: None,
            capabilities: None,
            content_hints: None,
            content_restrictions: None,
            copy_requires_writer_permission: None,
            created_time: None,
            description: None,
            drive_id: None,
            explicitly_trashed: None,
            export_links: None,
            file_extension: None,
            folder_color_rgb: None,
            full_file_extension: None,
            has_augmented_permissions: None,
            has_thumbnail: None,
            head_revision_id: None,
            icon_link: None,
            id: None,
            image_media_metadata: None,
            is_app_authorized: None,
            kind: None,
            last_modifying_user: None,
            link_share_metadata: None,
            md5_checksum: None,
            mime_type: None,
            modified_by_me: None,
            modified_by_me_time: None,
            modified_time: None,
            name: None,
            original_filename: None,
            owned_by_me: None,
            owners: None,
            parents: None,
            permission_ids: None,
            permissions: None,
            properties: None,
            quota_bytes_used: None,
            resource_key: None,
            shared: None,
            shared_with_me_time: None,
            sharing_user: None,
            shortcut_details: None,
            size: None,
            spaces: None,
            starred: None,
            team_drive_id: None,
            thumbnail_link: None,
            thumbnail_version: None,
            trashed: None,
            trashed_time: None,
            trashing_user: None,
            version: None,
            video_media_metadata: None,
            viewed_by_me: None,
            viewed_by_me_time: None,
            viewers_can_copy_content: None,
            web_content_link: None,
            web_view_link: None,
            writers_can_share: None,
        }
    }
}


