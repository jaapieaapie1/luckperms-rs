#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub key: String,
    #[serde(rename = "type")]
    pub type_: NodeType,
    pub value: bool,
    pub context: Vec<String>,
    pub expiry: Option<u64>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    RegexPermission,
    Inheritance,
    Prefix,
    Suffix,
    Meta,
    Weight,
    DisplayName,
}