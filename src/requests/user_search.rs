use url::UrlQuery;
use crate::models::NodeType;

pub enum SearchRequest {
    Key {
        key: String,
        type_: Option<NodeType>,
    },
    KeyStartsWith {
        key_starts_with: String,
        type_: Option<NodeType>,
    },
    MetaKey {
        meta_key: String,
        type_: Option<NodeType>,
    },
}

impl SearchRequest {
    pub fn new_key(key: String) -> Self {
        Self::Key {
            key,
            type_: None,
        }
    }

    pub fn new_key_type(key: String, type_: Option<NodeType>) -> Self {
        Self::Key {
            key,
            type_,
        }
    }

    pub fn new_key_starts_with(key_starts_with: String) -> Self {
        Self::KeyStartsWith {
            key_starts_with,
            type_: None,
        }
    }

    pub fn new_key_starts_with_type(key_starts_with: String, type_: Option<NodeType>) -> Self {
        Self::KeyStartsWith {
            key_starts_with,
            type_,
        }
    }

    pub fn new_meta_key(meta_key: String) -> Self {
        Self::MetaKey {
            meta_key,
            type_: None,
        }
    }

    pub fn new_meta_key_type(meta_key: String, type_: Option<NodeType>) -> Self {
        Self::MetaKey {
            meta_key,
            type_,
        }
    }

    pub fn set_query_values(&self, mut query: form_urlencoded::Serializer<'_, UrlQuery<'_>>) {
        match self {
            Self::Key { key, type_ } => {
                query.append_pair("key", key);
                if let Some(type_) = type_ {
                    query.append_pair("type", &type_.to_string());
                }
            },
            Self::KeyStartsWith { key_starts_with, type_ } => {
                query.append_pair("keyStartsWith", key_starts_with);
                if let Some(type_) = type_ {
                    query.append_pair("type", &type_.to_string());
                }
            },
            Self::MetaKey { meta_key, type_ } => {
                query.append_pair("metaKey", meta_key);
                if let Some(type_) = type_ {
                    query.append_pair("type", &type_.to_string());
                }
            },
        }
    }
}