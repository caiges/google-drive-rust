/*
 * Drive API
 *
 * Manages files in Drive including uploading, downloading, searching, detecting changes, and updating sharing permissions.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`drive_permissions_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DrivePermissionsCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_permissions_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DrivePermissionsDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_permissions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DrivePermissionsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_permissions_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DrivePermissionsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_permissions_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DrivePermissionsUpdateError {
    UnknownValue(serde_json::Value),
}


/// Creates a permission for a file or shared drive.
pub async fn drive_permissions_create(configuration: &configuration::Configuration, file_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, email_message: Option<&str>, enforce_single_parent: Option<bool>, move_to_new_owners_root: Option<bool>, send_notification_email: Option<bool>, supports_all_drives: Option<bool>, supports_team_drives: Option<bool>, transfer_ownership: Option<bool>, use_domain_admin_access: Option<bool>, permission: Option<crate::models::Permission>) -> Result<crate::models::Permission, Error<DrivePermissionsCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{fileId}/permissions", local_var_configuration.base_path, fileId=crate::apis::urlencode(file_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder = local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder = local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder = local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder = local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = email_message {
        local_var_req_builder = local_var_req_builder.query(&[("emailMessage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enforce_single_parent {
        local_var_req_builder = local_var_req_builder.query(&[("enforceSingleParent", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = move_to_new_owners_root {
        local_var_req_builder = local_var_req_builder.query(&[("moveToNewOwnersRoot", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = send_notification_email {
        local_var_req_builder = local_var_req_builder.query(&[("sendNotificationEmail", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_all_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsAllDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_team_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsTeamDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transfer_ownership {
        local_var_req_builder = local_var_req_builder.query(&[("transferOwnership", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_admin_access {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainAdminAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&permission);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DrivePermissionsCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a permission.
pub async fn drive_permissions_delete(configuration: &configuration::Configuration, file_id: &str, permission_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, supports_all_drives: Option<bool>, supports_team_drives: Option<bool>, use_domain_admin_access: Option<bool>) -> Result<(), Error<DrivePermissionsDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{fileId}/permissions/{permissionId}", local_var_configuration.base_path, fileId=crate::apis::urlencode(file_id), permissionId=crate::apis::urlencode(permission_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder = local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder = local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder = local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder = local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_all_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsAllDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_team_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsTeamDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_admin_access {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainAdminAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DrivePermissionsDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a permission by ID.
pub async fn drive_permissions_get(configuration: &configuration::Configuration, file_id: &str, permission_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, supports_all_drives: Option<bool>, supports_team_drives: Option<bool>, use_domain_admin_access: Option<bool>) -> Result<crate::models::Permission, Error<DrivePermissionsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{fileId}/permissions/{permissionId}", local_var_configuration.base_path, fileId=crate::apis::urlencode(file_id), permissionId=crate::apis::urlencode(permission_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder = local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder = local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder = local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder = local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_all_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsAllDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_team_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsTeamDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_admin_access {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainAdminAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DrivePermissionsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists a file's or shared drive's permissions.
pub async fn drive_permissions_list(configuration: &configuration::Configuration, file_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, include_permissions_for_view: Option<&str>, page_size: Option<i32>, page_token: Option<&str>, supports_all_drives: Option<bool>, supports_team_drives: Option<bool>, use_domain_admin_access: Option<bool>) -> Result<crate::models::PermissionList, Error<DrivePermissionsListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{fileId}/permissions", local_var_configuration.base_path, fileId=crate::apis::urlencode(file_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder = local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder = local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder = local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder = local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_permissions_for_view {
        local_var_req_builder = local_var_req_builder.query(&[("includePermissionsForView", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_all_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsAllDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_team_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsTeamDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_admin_access {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainAdminAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DrivePermissionsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a permission with patch semantics.
pub async fn drive_permissions_update(configuration: &configuration::Configuration, file_id: &str, permission_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, remove_expiration: Option<bool>, supports_all_drives: Option<bool>, supports_team_drives: Option<bool>, transfer_ownership: Option<bool>, use_domain_admin_access: Option<bool>, permission: Option<crate::models::Permission>) -> Result<crate::models::Permission, Error<DrivePermissionsUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{fileId}/permissions/{permissionId}", local_var_configuration.base_path, fileId=crate::apis::urlencode(file_id), permissionId=crate::apis::urlencode(permission_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder = local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder = local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder = local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder = local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = remove_expiration {
        local_var_req_builder = local_var_req_builder.query(&[("removeExpiration", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_all_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsAllDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supports_team_drives {
        local_var_req_builder = local_var_req_builder.query(&[("supportsTeamDrives", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transfer_ownership {
        local_var_req_builder = local_var_req_builder.query(&[("transferOwnership", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_admin_access {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainAdminAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&permission);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DrivePermissionsUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

