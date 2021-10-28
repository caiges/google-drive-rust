/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentRestriction : A restriction for accessing the content of the file.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentRestriction {
    /// Whether the content of the file is read-only. If a file is read-only, a new revision of the file may not be added, comments may not be added or modified, and the title of the file may not be modified.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Reason for why the content of the file is restricted. This is only mutable on requests that also set readOnly=true.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "restrictingUser", skip_serializing_if = "Option::is_none")]
    pub restricting_user: Option<Box<crate::models::User>>,
    /// The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true.
    #[serde(rename = "restrictionTime", skip_serializing_if = "Option::is_none")]
    pub restriction_time: Option<String>,
    /// The type of the content restriction. Currently the only possible value is globalContentRestriction.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ContentRestriction {
    /// A restriction for accessing the content of the file.
    pub fn new() -> ContentRestriction {
        ContentRestriction {
            read_only: None,
            reason: None,
            restricting_user: None,
            restriction_time: None,
            _type: None,
        }
    }
}


