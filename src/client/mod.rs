pub mod response;

pub use crate::client::response::*;

use crate::message::Message;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, RETRY_AFTER};
use reqwest::{Body, StatusCode};

/// An async client for sending the notification payload.
pub struct Client {
    http_client: reqwest::Client,
    project_id: String,
    token: String,
}

impl Default for Client {
    fn default() -> Self {
        Self::new("".to_string(), "".to_string())
    }
}

impl Client {
    /// Get a new instance of Client.
    pub fn new(project_id: String, token: String) -> Client {
        let http_client = reqwest::ClientBuilder::new()
            .pool_max_idle_per_host(std::usize::MAX)
            .build()
            .unwrap();

        Client { http_client, project_id, token }
    }

    /// Try sending a `Message` to FCM.
    pub async fn send(&self, message: Message<'_>) -> Result<FcmResponse, FcmError> {
        let payload = serde_json::to_vec(&message).unwrap();

        let request = self
            .http_client
            .post(format!("https://fcm.googleapis.com/v1/projects/{}/messages:send", self.project_id))
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, format!("{}", payload.len() as u64).as_bytes())
            .header(AUTHORIZATION, format!("Bearer {}", self.token).as_bytes())
            .body(Body::from(payload))
            .build()?;
        let response = self.http_client.execute(request).await?;

        let response_status = response.status();

        let retry_after = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|ra| ra.to_str().ok())
            .and_then(|ra| ra.parse::<RetryAfter>().ok());

        match response_status {
            StatusCode::OK => {
                let fcm_response: FcmResponse = response.json().await.unwrap();

                match fcm_response.error {
                    Some(ErrorReason::Unavailable) => Err(response::FcmError::ServerError(retry_after)),
                    Some(ErrorReason::InternalServerError) => Err(response::FcmError::ServerError(retry_after)),
                    _ => Ok(fcm_response),
                }
            }
            StatusCode::UNAUTHORIZED => Err(response::FcmError::Unauthorized),
            StatusCode::BAD_REQUEST => Err(response::FcmError::InvalidMessage("Bad Request".to_string())),
            status if status.is_server_error() => Err(response::FcmError::ServerError(retry_after)),
            _ => Err(response::FcmError::InvalidMessage("Unknown Error".to_string())),
        }
    }
}
