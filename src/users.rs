use reqwest::StatusCode;
use uuid::Uuid;
use crate::{LuckClient, models};
use crate::errors::RequestError;
use crate::models::{PermissionCheckResult, TrackMoveRequest, TrackMoveResponse, User, UserIdentifier, UsernameUpdateRequest, UserSearchResult};
use crate::requests::PermissionCheckRequest;

impl LuckClient {
    /// Get a list of all users on the LuckPerms instance.
    pub async fn users(&self) -> Result<Vec<Uuid>, RequestError> {
        let url = self.base_url.join("/user")?;
        let response = self.client.get(url).send().await?;
        let users: Vec<Uuid> = response.json().await?;
        Ok(users)
    }

    /// Create a new user on the LuckPerms instance.
    pub async fn create_user(&self, user: UserIdentifier) -> Result<User, RequestError> {
        let url = self.base_url.join("/user")?;
        let response = self.client.post(url).json(&user).send().await?;
        let user: User = response.error_for_status()?.json().await?;

        Ok(user)
    }

    /// Lookup a user based on a username from the LuckPerms instance.
    pub async fn username_lookup(&self, username: String) -> Result<UserIdentifier, RequestError> {
        let mut url = self.base_url.join("/user/lookup")?;
        url.query_pairs_mut().append_pair("username", &username);
        let response = self.client.get(url).send().await?;
        let user: UserIdentifier = response.error_for_status()?.json().await?;

        Ok(user)
    }

    /// Lookup a user based on a UUID from the LuckPerms instance.
    pub async fn uuid_lookup(&self, uuid: Uuid) -> Result<UserIdentifier, RequestError> {
        let mut url = self.base_url.join("/user/lookup")?;
        url.query_pairs_mut().append_pair("uniqueId", &uuid.to_string());
        let response = self.client.get(url).send().await?;
        let user: UserIdentifier = response.error_for_status()?.json().await?;

        Ok(user)
    }

    /// Search for users matching certain nodes.
    pub async fn user_search(&self, search: crate::requests::SearchRequest) -> Result<Vec<UserSearchResult>, RequestError> {
        let mut url = self.base_url.join("/user/search")?;
        let query = url.query_pairs_mut();
        search.set_query_values(query);
        let response = self.client.get(url).send().await?;
        let users: Vec<UserSearchResult> = response.error_for_status()?.json().await?;

        Ok(users)
    }

    /// Get a user based on their UUID.
    pub async fn get_user(&self, uuid: Uuid) -> Result<Option<User>, RequestError> {
        let url = self.base_url.join(&format!("/user/{}", uuid))?;
        let response = self.client.get(url).send().await?;

        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let user: User = response.error_for_status()?.json().await?;

        Ok(Some(user))
    }

    /// Update a user's username.
    pub async fn update_username(&self, uuid: Uuid, username: String) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/user/{}", uuid))?;
        let response = self.client.patch(url).json(&UsernameUpdateRequest {
            username,
        }).send().await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Delete a user from the LuckPerms instance.
    pub async fn delete_user(&self, uuid: Uuid) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/user/{}", uuid))?;
        let response = self.client.delete(url).send().await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Get all nodes for a user.
    pub async fn get_user_nodes(&self, uuid: Uuid) -> Result<Vec<models::Node>, RequestError> {
        let url = self.base_url.join(&format!("/user/{}/nodes", uuid))?;
        let response = self.client.get(url).send().await?;
        let nodes: Vec<models::Node> = response.error_for_status()?.json().await?;

        Ok(nodes)
    }

    /// Add a node to a user.
    pub async fn add_user_node(&self, uuid: Uuid, node: models::Node) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/user/{}/nodes", uuid))?;
        let response = self.client.post(url).json(&node).send().await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Add multiple nodes to a user.
    pub async fn add_user_nodes(&self, uuid: Uuid, nodes: Vec<models::Node>) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/user/{}/nodes", uuid))?;
        let response = self.client.patch(url).json(&nodes).send().await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Set a user's nodes.
    pub async fn set_user_nodes(&self, uuid: Uuid, nodes: Vec<models::Node>) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/user/{}/nodes", uuid))?;
        let response = self.client.put(url).json(&nodes).send().await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Delete nodes from a user.
    pub async fn delete_user_nodes(&self, uuid: Uuid, nodes: Vec<models::Node>) -> Result<(), RequestError> {
        let url = self.base_url.join(&format!("/user/{}/nodes", uuid))?;
        let response = self.client.delete(url).json(&nodes).send().await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Get a user's meta data.
    pub async fn get_user_metadata(&self, uuid: Uuid) -> Result<Vec<models::Metadata>, RequestError> {
        let url = self.base_url.join(&format!("/user/{}/meta", uuid))?;
        let response = self.client.get(url).send().await?;
        let metadata: Vec<models::Metadata> = response.error_for_status()?.json().await?;

        Ok(metadata)
    }

    /// Check if a user has a permission.
    pub async fn check_user_permission(&self, uuid: Uuid, permission: String) -> Result<PermissionCheckResult, RequestError> {
        let mut url = self.base_url.join(&format!("/user/{}/permissionCheck", uuid))?;
        url.query_pairs_mut().append_pair("permission", &permission);
        let response = self.client.get(url).send().await?;
        let result: PermissionCheckResult = response.error_for_status()?.json().await?;

        Ok(result)
    }

    /// Check if a user has a permission with more specific query options.
    pub async fn check_user_permission_query(&self, uuid: Uuid, request: PermissionCheckRequest) -> Result<PermissionCheckResult, RequestError> {
        let url = self.base_url.join(&format!("/user/{}/permissionCheck", uuid))?;
        let response = self.client.post(url).json(&request).send().await?;
        let result: PermissionCheckResult = response.error_for_status()?.json().await?;

        Ok(result)
    }

    /// Promote a user along a track.
    pub async fn promote_user(&self, uuid: Uuid, track: String) -> Result<TrackMoveResponse, RequestError> {
        let url = self.base_url.join(&format!("/user/{}/promote", uuid))?;
        let response = self.client.post(url).json(&TrackMoveRequest {
            track,
        }).send().await?;



        Ok(response.error_for_status()?.json().await?)
    }

    /// Demote a user along a track.
    pub async fn demote_user(&self, uuid: Uuid, track: String) -> Result<TrackMoveResponse, RequestError> {
        let url = self.base_url.join(&format!("/user/{}/demote", uuid))?;
        let response = self.client.post(url).json(&TrackMoveRequest {
            track,
        }).send().await?;

        Ok(response.error_for_status()?.json().await?)
    }
}