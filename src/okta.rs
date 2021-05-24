use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct OktaConfig {
    pub client_id: String,
    pub url: String,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OktaAuthnResponse {
    #[serde(rename = "expiresAt")]
    expires_at: String,
    status: String,
    #[serde(rename = "sessionToken")]
    session_token: String,
}

pub struct PKCEChallenge {
    code_verifier: String,
    code_challenge: String
}

pub struct OpenIDConfig {
    authorization_endpoint: String,
    token_endpoint: String
}

/// Use a username and password to get an OKTA Session Token
pub async fn do_okta_authn(username: String, password: String, config: &OktaConfig) -> Result<OktaAuthnResponse, String> {

    // Use Username and Password to get a Session Token
    let client = reqwest::Client::new();

    let json = serde_json::json!({
        "username": username,
        "password": password
    });

    // Post to /authn
    let req = client.post(format!("{}/api/v1/authn", config.url)).json(&json).send().await;
    let res = req.expect("Error, got bad or no reply from /api/v1/authn");

    // Deserialize
    match res.json::<OktaAuthnResponse>().await {
        Ok(response) => Ok(response),
        Err(_) => Err(String::from("Couldn't understand /api/v1/authn response"))
    }

}

/// Use a Session Token to get an auth code
pub async fn do_okta_authorize(session_token: String, pkce: &PKCEChallenge, authorization_endpoint: String, config: &OktaConfig) -> Result<String, String> {

    let client = reqwest::Client::new();

    // Use Session Token to get an Auth Code
    let params = [
        ("client_id", config.client_id.to_string()),
        ("response_type", "code".to_string()),
        ("code_challenge_method", "S256".to_string()),
        ("code_challenge", String::from(&pkce.code_challenge)),
        ("redirect_uri", config.redirect_uri.to_string()),
        ("scope", "openid email groups profile".to_string()),
        ("prompt", "none".to_string()),
        ("response_mode", "form_post".to_string()),
        ("state", "a".to_string()),
        ("nonce", "a".to_string()),
        ("sessionToken", String::from(session_token))
    ];
    let url = reqwest::Url::parse_with_params(&authorization_endpoint, &params).expect("Failed to create URL");

    // Get Text Response to parse HTML for the code response
    let req = client.get(url).send().await;
    let text = req.expect("Error getting Code").text().await;
    let text = text.expect("Invalid Text");

    // Scrape code from <input name='code' value='....' />
    let dom = scraper::Html::parse_document(&text);
    let selector = scraper::Selector::parse(r#"input[name="code"]"#).unwrap();
    Ok(String::from(dom.select(&selector).next().expect("Missing Input with code- maybe got an error instead").value().attr("value").expect("Missing value on code")))

}

/// Use a Session Token to get an auth code
pub async fn do_okta_token(auth_code: String, pkce: &PKCEChallenge, token_endpoint: String, config: &OktaConfig) -> Result<String, String> {

    // Use Auth Code to get Access Token
    let form = [
        ("client_id", config.client_id.to_string()),
        ("code_verifier", pkce.code_verifier.to_string()),
        ("redirect_uri", config.redirect_uri.to_string()),
        ("grant_type", "authorization_code".to_string()),
        ("code", String::from(auth_code))
    ];

    let client = reqwest::Client::new();

    let req = client.post(token_endpoint).form(&form).send().await;
    let json = req.expect("Error getting Access Token").json::<HashMap<String, serde_json::Value>>().await;
    let json = json.expect("Invalid JSON");
    Ok(String::from(json["access_token"].as_str().expect("Missing access token")))

}

pub async fn get_openid_config(url: String) -> Result<OpenIDConfig, String> {

    // Get config from well-known
    let req = reqwest::get(url).await.expect("Error Getting URL");
    let res = req.json::<HashMap<String, serde_json::Value>>().await;
    let res = res.expect("Error parsing JSON");

    let token_endpoint = res["token_endpoint"].as_str().expect("Missing token_endpoint");
    let authorization_endpoint = res["authorization_endpoint"].as_str().expect("Missing authorization_endpoint");

    Ok(OpenIDConfig { token_endpoint: String::from(token_endpoint), authorization_endpoint: String::from(authorization_endpoint) })

}

#[tokio::main]
pub async fn get_access_token(username: String, password: String, config: OktaConfig) -> Result<String, String> {

    // Get config from .well-known
    let openid_config = get_openid_config(format!("{}/oauth2/default/.well-known/openid-configuration", config.url)).await?;

    // Create a code verifier and code challenge
    let code_verifier = pkce::code_verifier(128);
    let pkce = PKCEChallenge {
        code_verifier: String::from_utf8(code_verifier.clone()).expect("Couldn't convert from vec to string"),
        code_challenge: pkce::code_challenge(&code_verifier)
    };

    // Get Session token from /authn
    let okta_session = do_okta_authn(username, password, &config).await?;

    // Get Auth Code from /authorization
    let auth_code = do_okta_authorize(okta_session.session_token, &pkce, openid_config.authorization_endpoint, &config).await?;

    // Get Access Token from /token
    Ok(do_okta_token(auth_code, &pkce, openid_config.token_endpoint, &config).await?)
}

