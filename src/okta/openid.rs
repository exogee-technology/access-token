use crate::okta::error::OktaClientError;

pub struct OpenIDConfig {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

/// Get OpenID config from .well-known
#[tokio::main]
pub async fn get_openid_config(base_url: String) -> Result<OpenIDConfig, OktaClientError> {
    let url = format!("{}/oauth2/default/.well-known/openid-configuration", base_url);

    let req = reqwest::get(url).await.expect("Error Getting URL");
    let res = req.json::<std::collections::HashMap<String, serde_json::Value>>().await;
    let res = res.expect("Error parsing JSON");

    let token_endpoint = res["token_endpoint"].as_str().expect("Missing token_endpoint");
    let authorization_endpoint = res["authorization_endpoint"].as_str().expect("Missing authorization_endpoint");

    Ok(OpenIDConfig { token_endpoint: token_endpoint.to_owned(), authorization_endpoint: authorization_endpoint.to_owned() })
}

