/// Call the OAuth token endpoint
use crate::OktaClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    Implicit, // Not Implemented
    AuthorizationCode,
    ClientCredentials, // Not Implemented
    Password,          // Not Implemented
    RefreshToken,      // Not Implemented
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaTokenRequest {
    pub client_id: String,
    pub code_verifier: String,
    pub redirect_uri: String,
    pub grant_type: GrantType,
    pub code: String,
}

impl OktaClient {
    /// Use an auth code to get an access token
    pub async fn do_oauth_token(&self, auth_code: String) -> Result<String, String> {
        let request = OktaTokenRequest {
            client_id: self.client_id.to_owned(),
            code_verifier: self.pkce.code_verifier.to_owned(),
            redirect_uri: self.login_redirect_url.to_owned(),
            grant_type: GrantType::AuthorizationCode,
            code: auth_code.to_owned(),
        };

        let client = reqwest::Client::new();

        let req = client
            .post(&self.token_endpoint)
            .form(&request)
            .send()
            .await;

        let json = req
            .expect("Error getting Access Token")
            .json::<std::collections::HashMap<String, serde_json::Value>>()
            .await;

        let json = json.expect("Invalid JSON");

        Ok(json["access_token"]
            .as_str()
            .expect("Missing access token")
            .to_owned())
    }
}
