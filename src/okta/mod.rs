mod openid;
mod pkce;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaAuthnRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaAuthnResponse {
    #[serde(rename = "expiresAt")]
    expires_at: String,
    status: String,
    #[serde(rename = "sessionToken")]
    session_token: String,
}

#[derive(Debug)]
pub struct OktaAuthorizeRequest {
    client_id: String,
    response_type: String,
    code_challenge_method: String,
    code_challenge: String,
    redirect_uri: String,
    scope: String,
    prompt: String,
    response_mode: String,
    state: String,
    nonce: String,
    session_token: String,
}

impl OktaAuthorizeRequest {
    pub fn as_params(&self) -> Vec<(&str, String)> {
        vec![
            ("client_id", self.client_id.to_owned()),
            ("response_type", self.response_type.to_owned()),
            ("code_challenge_method", self.code_challenge_method.to_owned()),
            ("code_challenge", self.code_challenge.to_owned()),
            ("redirect_uri", self.redirect_uri.to_owned()),
            ("scope", self.scope.to_owned()),
            ("prompt", self.prompt.to_owned()),
            ("response_mode", self.response_mode.to_owned()),
            ("state", self.state.to_owned()),
            ("sessionToken", self.session_token.to_owned()),
            ("nonce", self.nonce.to_owned()),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaTokenRequest {
    client_id: String,
    code_verifier: String,
    redirect_uri: String,
    grant_type: String,
    code:  String
}

pub struct OktaClient {
    client_id: String,
    base_url: String,
    login_redirect_url: String,
    username: String,
    password: String,
    authorization_endpoint: String,
    token_endpoint: String,
    pkce: pkce::PKCE,
    scopes: String,
}

impl OktaClient {
    /// Create a new OKTA Client
    pub fn new(username: String, password: String, client_id: String, login_redirect_url: String, base_url: String, scopes: String) -> Result<Self, String> {
        let openid_config = openid::get_openid_config(base_url.to_owned())?;

        Ok(OktaClient {
            username,
            password,
            client_id,
            login_redirect_url,
            base_url,
            authorization_endpoint: openid_config.authorization_endpoint,
            token_endpoint: openid_config.token_endpoint,
            pkce: pkce::PKCE::new(),
            scopes,
        })
    }

    /// Use a username and password to get a session token
    pub async fn do_okta_authn(&self) -> Result<OktaAuthnResponse, String> {

        let request = OktaAuthnRequest {
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        };

        let client = reqwest::Client::new();

        // Post to /authn
        let req = client
            .post(format!("{}/api/v1/authn", self.base_url))
            .json(&request)
            .send().await;

        let res = req.expect("Error, got bad or no reply from /api/v1/authn");

        // Deserialize
        match res.json::<OktaAuthnResponse>().await {
            Ok(response) => Ok(response),
            Err(_) => Err("Couldn't understand /api/v1/authn response".to_owned())
        }
    }

    /// Use a Session Token to get an auth code
    pub async fn do_okta_authorize(&self, session_token: String) -> Result<String, String> {

        let request = OktaAuthorizeRequest {
            client_id: self.client_id.to_owned(),
            response_type: "code".to_owned(),
            code_challenge_method: "S256".to_owned(),
            code_challenge: self.pkce.code_challenge.to_owned(),
            redirect_uri: self.login_redirect_url.to_owned(),
            scope: self.scopes.to_owned(),
            prompt: "none".to_owned(),
            response_mode: "form_post".to_owned(),
            state: "a".to_owned(),
            nonce: "a".to_owned(),
            session_token: session_token.to_owned(),
        };

        let url = reqwest::Url::parse_with_params(&self.authorization_endpoint, &request.as_params())
            .expect("Failed to create URL");

        let client = reqwest::Client::new();

        // Get Text Response to parse HTML for the code response
        let req = client.get(url).send().await;
        let text = req.expect("Error getting Code").text().await;
        let text = text.expect("Invalid Text");

        // Scrape code from <input name='code' value='....' />
        let dom = scraper::Html::parse_document(&text);
        let selector = scraper::Selector::parse(r#"input[name="code"]"#).unwrap();
        Ok(dom.select(&selector).next()
            .expect("Missing Input with code- maybe got an error instead")
            .value().attr("value").expect("Missing value on code").to_owned())
    }

    /// Use an auth code to get an access token
    pub async fn do_okta_token(&self, auth_code: String) -> Result<String, String> {

        let request = OktaTokenRequest {
            client_id: self.client_id.to_owned(),
            code_verifier: self.pkce.code_verifier.to_owned(),
            redirect_uri: self.login_redirect_url.to_owned(),
            grant_type: "authorization_code".to_owned(),
            code: auth_code.to_owned()
        };

        let client = reqwest::Client::new();

        let req = client.post(&self.token_endpoint).form(&request).send().await;
        let json = req
            .expect("Error getting Access Token")
            .json::<std::collections::HashMap<String, serde_json::Value>>().await;

        let json = json.expect("Invalid JSON");

        Ok(json["access_token"].as_str().expect("Missing access token").to_owned())
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