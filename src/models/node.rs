#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub key: String,
    #[serde(rename = "type")]
    pub type_: NodeType,
    pub value: bool,
    pub context: Vec<String>,
    pub expiry: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

impl NodeType {
    pub fn to_string(&self) -> String {
        match self {
            Self::RegexPermission => "regex_permission".to_string(),
            Self::Inheritance => "inheritance".to_string(),
            Self::Prefix => "prefix".to_string(),
            Self::Suffix => "suffix".to_string(),
            Self::Meta => "meta".to_string(),
            Self::Weight => "weight".to_string(),
            Self::DisplayName => "display_name".to_string(),
        }
    }
}