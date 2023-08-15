use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timestamp: Option<u64>,
    pub source: ActionSource,
    pub target: ActionTarget,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSource {
    pub unique_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unique_id: Option<Uuid>,
    pub name: String,
    #[serde(rename = "type")]
    pub target_type: ActionTargetType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionTargetType {
    User,
    Group,
    Track,
}