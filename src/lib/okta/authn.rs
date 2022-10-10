/// Call the OKTA /api/v1/authn endpoint
use crate::okta::{OktaClient, OktaClientError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OktaAuthnRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct OktaAuthnResponse {
    pub expires_at: Option<String>,
    pub status: Option<String>,
    pub session_token: Option<String>,
    pub error_code: Option<String>,
    pub error_summary: Option<String>,
    pub error_id: Option<String>,
}

impl OktaAuthnResponse {
    fn as_error(&self) -> Option<OktaClientError> {
        match self.error_code {
            None => None,
            _ => Some(OktaClientError::OktaAPI(
                self.error_summary.as_ref().unwrap().to_owned(),
            )),
        }
    }
}

impl Default for OktaAuthnResponse {
    fn default() -> Self {
        OktaAuthnResponse {
            expires_at: None,
            status: None,
            session_token: None,
            error_code: None,
            error_summary: None,
            error_id: None,
        }
    }
}

impl OktaClient {
    /// Use a username and password to get a session token
    pub async fn do_okta_authn(&self) -> Result<OktaAuthnResponse, OktaClientError> {
        let request = OktaAuthnRequest {
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        };

        let client = reqwest::Client::new();

        // Post to /authn
        let req = client
            .post(format!("{}/api/v1/authn", self.base_url))
            .json(&request)
            .send()
            .await?;

        // Deserialize
        let response = req.json::<OktaAuthnResponse>().await?;

        match response.as_error() {
            None => Ok(response),
            Some(e) => Err(e),
        }
    }
}
