use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct PKCE {
    pub code_verifier: String,
    pub code_challenge: String
}

impl PKCE {

    /// Create a new Code Verifier
    pub fn new() -> Self {
        let code_verifier = pkce::code_verifier(128);
        PKCE {
            code_verifier: String::from_utf8(code_verifier.clone()).expect("Couldn't convert from vec to string"),
            code_challenge: pkce::code_challenge(&code_verifier)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaAuthnResponse {
    #[serde(rename = "expiresAt")]
    expires_at: String,
    status: String,
    #[serde(rename = "sessionToken")]
    session_token: String,
}

pub struct OpenIDConfig {
    authorization_endpoint: String,
    token_endpoint: String,
}

pub struct OktaClient {
    client_id: String,
    base_url: String,
    login_redirect_url: String,
    username: String,
    password: String,
    authorization_endpoint: String,
    token_endpoint: String,
    pkce: PKCE,
    scopes: String
}

impl OktaClient {

    /// Create a new OKTA Client
    pub fn new(username: String, password: String, client_id: String, login_redirect_url: String, base_url: String, scopes: String) -> Result<Self, String> {

        let openid_config = get_openid_config(String::from(&base_url))?;

        Ok(OktaClient {
            username,
            password,
            client_id,
            login_redirect_url,
            base_url,
            authorization_endpoint: openid_config.authorization_endpoint,
            token_endpoint: openid_config.token_endpoint,
            pkce: PKCE::new(),
            scopes
        })
    }

    /// Use a username and password to get a session token
    pub async fn do_okta_authn(&self) -> Result<OktaAuthnResponse, String> {

        let client = reqwest::Client::new();

        let json = serde_json::json!({
            "username": self.username,
            "password": self.password
        });

        // Post to /authn
        let req = client.post(format!("{}/api/v1/authn", self.base_url)).json(&json).send().await;
        let res = req.expect("Error, got bad or no reply from /api/v1/authn");

        // Deserialize
        match res.json::<OktaAuthnResponse>().await {
            Ok(response) => Ok(response),
            Err(_) => Err(String::from("Couldn't understand /api/v1/authn response"))
        }
    }

    /// Use a Session Token to get an auth code
    pub async fn do_okta_authorize(&self, session_token: String) -> Result<String, String> {
        let client = reqwest::Client::new();

        let params = [
            ("client_id", String::from(&self.client_id)),
            ("response_type", String::from("code")),
            ("code_challenge_method", String::from("S256")),
            ("code_challenge", String::from(&self.pkce.code_challenge)),
            ("redirect_uri", String::from(&self.login_redirect_url)),
            ("scope", String::from(&self.scopes)),
            ("prompt", String::from("none")),
            ("response_mode", String::from("form_post")),
            ("state", String::from("a")),
            ("nonce", String::from("a")),
            ("sessionToken", String::from(session_token))
        ];

        let url = reqwest::Url::parse_with_params(&self.authorization_endpoint, &params)
            .expect("Failed to create URL");

        // Get Text Response to parse HTML for the code response
        let req = client.get(url).send().await;
        let text = req.expect("Error getting Code").text().await;
        let text = text.expect("Invalid Text");

        // Scrape code from <input name='code' value='....' />
        let dom = scraper::Html::parse_document(&text);
        let selector = scraper::Selector::parse(r#"input[name="code"]"#).unwrap();
        Ok(String::from(dom.select(&selector).next()
            .expect("Missing Input with code- maybe got an error instead")
            .value().attr("value").expect("Missing value on code")))
    }

    /// Use an auth code to get an access token
    pub async fn do_okta_token(&self, auth_code: String) -> Result<String, String> {

        let form = [
            ("client_id", String::from(&self.client_id)),
            ("code_verifier", String::from(&self.pkce.code_verifier)),
            ("redirect_uri", String::from(&self.login_redirect_url)),
            ("grant_type", String::from("authorization_code")),
            ("code", String::from(auth_code))
        ];

        let client = reqwest::Client::new();

        let req = client.post(&self.token_endpoint).form(&form).send().await;
        let json = req.expect("Error getting Access Token").json::<HashMap<String, serde_json::Value>>().await;
        let json = json.expect("Invalid JSON");
        Ok(String::from(json["access_token"].as_str().expect("Missing access token")))
    }

    /// Get an access token for a specific OKTA tenant and client/app
    #[tokio::main]
    pub async fn get_access_token(&self) -> Result<String, String> {

        // Get Session token from /authn
        let okta_session = self.do_okta_authn().await?;

        // Get Auth Code from /authorization
        let auth_code = self.do_okta_authorize(okta_session.session_token).await?;

        // Get Access Token from /token
        let token = self.do_okta_token(auth_code).await?;

        Ok(token)
    }
}

/// Get OpenID config from .well-known
#[tokio::main]
pub async fn get_openid_config(base_url: String) -> Result<OpenIDConfig, String> {

    let url = format!("{}/oauth2/default/.well-known/openid-configuration", base_url);

    let req = reqwest::get(url).await.expect("Error Getting URL");
    let res = req.json::<HashMap<String, serde_json::Value>>().await;
    let res = res.expect("Error parsing JSON");

    let token_endpoint = res["token_endpoint"].as_str().expect("Missing token_endpoint");
    let authorization_endpoint = res["authorization_endpoint"].as_str().expect("Missing authorization_endpoint");

    Ok(OpenIDConfig { token_endpoint: String::from(token_endpoint), authorization_endpoint: String::from(authorization_endpoint) })
}

