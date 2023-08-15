use reqwest::StatusCode;
use crate::errors::RequestError;
use crate::LuckClient;
use crate::models::{GroupCreateRequest, Group, GroupSearchResult, Node, Metadata, PermissionCheckResult};
use crate::requests::{PermissionCheckRequest, SearchRequest};

impl LuckClient {
    /// Get all group names.
    pub async fn get_groups(&self) -> Result<Vec<String>, RequestError> {
        let url = self.base_url.join("/group")?;
        let response = self.client.get(url).send().await?;
        let groups: Vec<String> = response.json().await?;
        Ok(groups)
    }

    /// Create a new group.
    pub async fn create_group(&self, name: String) -> Result<Group, RequestError> {
        let url = self.base_url.join("/group")?;
        let response = self.client.post(url).json(&GroupCreateRequest {name}).send().await?;
        Ok(response.error_for_status()?.json().await?)
    }

    /// Search for a group matching nodes.
    pub async fn search_group(&self, search_request: SearchRequest) -> Result<Vec<GroupSearchResult>, RequestError> {
        let mut url = self.base_url.join("/group/search")?;
        let query = url.query_pairs_mut();
        search_request.set_query_values(query);
        let response = self.client.get(url).send().await?;
        let groups: Vec<GroupSearchResult> = response.error_for_status()?.json().await?;

        Ok(groups)
    }

    /// Get a group based on its name.
    pub async fn get_group(&self, name: String) -> Result<Option<Group>, RequestError> {
        let url = self.base_url.join(&format!("/group/{}", name))?;
        let response = self.client.get(url).send().await?;

        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let group: Group = response.error_for_status()?.json().await?;

        Ok(Some(group))
    }

    /// Delete a group based on its name.
    pub async fn delete_group(&self, name: String) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/group/{}", name))?;
        let response = self.client.delete(url).send().await?;
        response.error_for_status()?;
        Ok(())
    }

    /// Get a group's notes.
    pub async fn get_group_nodes(&self, name: String) -> Result<Vec<Node>, RequestError> {
        let url = self.base_url.join(&format!("/group/{}/nodes", name))?;
        let response = self.client.get(url).send().await?;
        let nodes: Vec<Node> = response.json().await?;
        Ok(nodes)
    }

    /// Add a node to a group.
    pub async fn add_group_node(&self, name: String, node: Node) -> Result<Vec<Node>, RequestError> {
        let url = self.base_url.join(&format!("/group/{}/nodes", name))?;
        let response = self.client.post(url).json(&node).send().await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    /// Add multiple nodes to a group.
    pub async fn add_group_nodes(&self, name: String, nodes: Vec<Node>) -> Result<Vec<Node>, RequestError> {
        let url = self.base_url.join(&format!("/group/{}/nodes", name))?;
        let response = self.client.patch(url).json(&nodes).send().await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    /// Set nodes for a group.
    pub async fn set_group_nodes(&self, name: String, nodes: Vec<Node>) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/group/{}/nodes", name))?;
        let response = self.client.put(url).json(&nodes).send().await?;
        response.error_for_status()?;
        Ok(())
    }

    /// Delete nodes from a group.
    pub async fn delete_group_node(&self, name: String, nodes: Vec<Node>) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/group/{}/nodes", name))?;
        let response = self.client.delete(url).json(&nodes).send().await?;
        response.error_for_status()?;
        Ok(())
    }

    /// Get a group's metadata.
    pub async fn get_group_metadata(&self, name: String) -> Result<Metadata, RequestError> {
        let url = self.base_url.join(&format!("/group/{}/meta", name))?;
        let response = self.client.get(url).send().await?;
        let meta: Metadata = response.json().await?;
        Ok(meta)
    }

    /// Check if a group has a permission.
    pub async fn check_group_permission(&self, name: String, permission: String) -> Result<PermissionCheckResult, RequestError> {
        let mut url = self.base_url.join(&format!("/group/{}/permissionCheck", name))?;
        url.query_pairs_mut().append_pair("permission", &permission);
        let response = self.client.get(url).send().await?;
        let result: PermissionCheckResult = response.json().await?;
        Ok(result)
    }

    /// Check if a group has a permission with advanced query settings.
    pub async fn check_group_permission_query(&self, name: String, request: PermissionCheckRequest) -> Result<PermissionCheckResult, RequestError> {
        let url = self.base_url.join(&format!("/group/{}/permissionCheck", name))?;
        let response = self.client.post(url).json(&request).send().await?;
        let result: PermissionCheckResult = response.error_for_status()?.json().await?;

        Ok(result)
    }
}