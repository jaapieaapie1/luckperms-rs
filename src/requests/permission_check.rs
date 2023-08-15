use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionCheckRequest {
    pub permission: String,
    pub query_options: QueryOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<QueryMode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<QueryFlag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<Context>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMode {
    Contextual,
    NonContextual,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryFlag {
    ResolveInheritance,
    IncludeNodesWithoutServerContext,
    IncludeNodesWithoutWorldContext,
    ApplyInheritanceNodesWithoutServerContext,
    ApplyInheritanceNodesWithoutWorldContext,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub key: String,
    pub value: String,
}