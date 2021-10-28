/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// User : Information about a Drive user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    /// A plain text displayable name for this user.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string \"drive#user\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Whether this user is the requesting user.
    #[serde(rename = "me", skip_serializing_if = "Option::is_none")]
    pub me: Option<bool>,
    /// The user's ID as visible in Permission resources.
    #[serde(rename = "permissionId", skip_serializing_if = "Option::is_none")]
    pub permission_id: Option<String>,
    /// A link to the user's profile photo, if available.
    #[serde(rename = "photoLink", skip_serializing_if = "Option::is_none")]
    pub photo_link: Option<String>,
}

impl User {
    /// Information about a Drive user.
    pub fn new() -> User {
        User {
            display_name: None,
            email_address: None,
            kind: None,
            me: None,
            permission_id: None,
            photo_link: None,
        }
    }
}

