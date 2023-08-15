use crate::errors::RequestError;
use crate::LuckClient;
use crate::models::Action;

impl LuckClient {
    /// Submit an action to the server.
    pub async fn submit_action(&self, action: Action) -> Result<(), RequestError> {
        let url = self.base_url.join("/action")?;
        let response = self.client.post(url).json(&action).send().await?;
        response.error_for_status()?;
        Ok(())
    }
}