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


/// struct for typed errors of method [`drive_drives_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_drives_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_drives_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_drives_hide`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesHideError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_drives_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_drives_unhide`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesUnhideError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drive_drives_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriveDrivesUpdateError {
    UnknownValue(serde_json::Value),
}


/// Creates a new shared drive.
pub async fn drive_drives_create(configuration: &configuration::Configuration, request_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, drive: Option<crate::models::Drive>) -> Result<crate::models::Drive, Error<DriveDrivesCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.query(&[("requestId", &request_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&drive);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DriveDrivesCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
pub async fn drive_drives_delete(configuration: &configuration::Configuration, drive_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>) -> Result<(), Error<DriveDrivesDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives/{driveId}", local_var_configuration.base_path, driveId=crate::apis::urlencode(drive_id));
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
        let local_var_entity: Option<DriveDrivesDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a shared drive's metadata by ID.
pub async fn drive_drives_get(configuration: &configuration::Configuration, drive_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, use_domain_admin_access: Option<bool>) -> Result<crate::models::Drive, Error<DriveDrivesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives/{driveId}", local_var_configuration.base_path, driveId=crate::apis::urlencode(drive_id));
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
        let local_var_entity: Option<DriveDrivesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Hides a shared drive from the default view.
pub async fn drive_drives_hide(configuration: &configuration::Configuration, drive_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>) -> Result<crate::models::Drive, Error<DriveDrivesHideError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives/{driveId}/hide", local_var_configuration.base_path, driveId=crate::apis::urlencode(drive_id));
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
        let local_var_entity: Option<DriveDrivesHideError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the user's shared drives.
pub async fn drive_drives_list(configuration: &configuration::Configuration, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, page_size: Option<i32>, page_token: Option<&str>, q: Option<&str>, use_domain_admin_access: Option<bool>) -> Result<crate::models::DriveList, Error<DriveDrivesListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives", local_var_configuration.base_path);
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
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = q {
        local_var_req_builder = local_var_req_builder.query(&[("q", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DriveDrivesListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Restores a shared drive to the default view.
pub async fn drive_drives_unhide(configuration: &configuration::Configuration, drive_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>) -> Result<crate::models::Drive, Error<DriveDrivesUnhideError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives/{driveId}/unhide", local_var_configuration.base_path, driveId=crate::apis::urlencode(drive_id));
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
        let local_var_entity: Option<DriveDrivesUnhideError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the metadate for a shared drive.
pub async fn drive_drives_update(configuration: &configuration::Configuration, drive_id: &str, alt: Option<&str>, fields: Option<&str>, key: Option<&str>, oauth_token: Option<&str>, pretty_print: Option<bool>, quota_user: Option<&str>, user_ip: Option<&str>, use_domain_admin_access: Option<bool>, drive: Option<crate::models::Drive>) -> Result<crate::models::Drive, Error<DriveDrivesUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/drives/{driveId}", local_var_configuration.base_path, driveId=crate::apis::urlencode(drive_id));
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
    local_var_req_builder = local_var_req_builder.json(&drive);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DriveDrivesUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

