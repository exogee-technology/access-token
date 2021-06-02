/// Call the OKTA /api/v1/authn endpoint

use crate::okta::OktaClient;
use serde::{Serialize, Deserialize};
use crate::okta::error::OktaClientError;

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaAuthnRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaAuthnResponse {
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    pub status: String,
    #[serde(rename = "sessionToken")]
    pub session_token: String,
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
            .send().await?;

        // Deserialize
        Ok(req.json::<OktaAuthnResponse>().await?)
    }

}