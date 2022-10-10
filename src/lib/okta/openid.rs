use crate::okta::OktaClientError;

use std::collections::{HashMap};
use scraper::{Selector, Html};

pub struct OpenIDConfig {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

/// Get OpenID config from .well-known
#[tokio::main]
pub async fn get_openid_config(
    base_url: String,
    authorization_server_id: String,
) -> Result<OpenIDConfig, OktaClientError> {
    let url = format!(
        "{}/oauth2/{}/.well-known/openid-configuration",
        base_url, authorization_server_id
    );

    let response = reqwest::get(&url).await.expect("Error Getting URL");
    let text = response.text().await.expect("Error getting text body");
    let json = serde_json::from_str::
        <HashMap<String, serde_json::Value>>(&text);

    let json = match json {
        Ok(json) => json,
        Err(_) => {

            // Look for a HTML response with error-code, o-form-explain
            let dom = Html::parse_document(&text);

            // Look for <div class="error-code">
            let error_code: String = dom.select(&Selector::parse(r#"div[class="error-code"]"#).unwrap())
                .flat_map(|element| element.text())
                .collect();

            // Look for <p class="o-form-explain">
            let error_explain: String = dom.select(&Selector::parse(r#"p[class="o-form-explain"]"#).unwrap())
                .flat_map(|element| element.text())
                .collect();

            // @todo return a nice error if code and explain are missing
            return Err(OktaClientError::Parser(format!("Error {} while getting the openid configuration at {}: {}", error_code, url, error_explain)))

        },
    };

    let token_endpoint = match json.get("token_endpoint") {
        Some(token_endpoint) => token_endpoint.as_str().expect("token_endpoint was expected to be a string"),
        // @todo capture JSON errors in this format:
        // {"errorCode":"E0000007","errorSummary":"Not found: Resource not found: ABCXYZ (AuthorizationServer)","errorLink":"E0000007","errorId":"123123","errorCauses":[]}

        None => return Err(OktaClientError::Parser(
            format!("Error while getting the openid configuration at {}: token_endpoint was missing from the openid configuration: {}",url, text))),
        };

    let authorization_endpoint = match json.get("authorization_endpoint") {
        Some(token_endpoint) => token_endpoint.as_str().expect("authorization_endpoint was expected to be a string"),
        None => return Err(OktaClientError::Parser(
        format!("Error while getting the openid configuration at {}: authorization_endpoint was missing from the openid configuration: {}", url, text))),
        };

    Ok(OpenIDConfig {
        token_endpoint: token_endpoint.to_owned(),
        authorization_endpoint: authorization_endpoint.to_owned(),
    })

}
