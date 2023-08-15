use std::collections::HashMap;
use uuid::Uuid;
use crate::models::node::Node;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentifier {
    pub unique_id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub unique_id: Uuid,
    pub username: String,
    #[serde(default)]
    pub parent_groups: Vec<String>,
    pub nodes: Vec<Node>,

}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub meta: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub primary_group: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResult {
    pub unique_id: Uuid,
    pub results: Vec<Node>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernameUpdateRequest {
    pub username: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionCheckResult {
    pub result: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node: Option<Node>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMoveRequest {
    pub track: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMoveResponse {
    pub success: bool,
    pub status: String,
    pub group_from: String,
    pub group_to: String,
}