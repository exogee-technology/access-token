use std::collections::HashMap;

pub struct OktaConfig {
    pub client_id: String,
    pub url: String,
    pub redirect_uri: String,
}

/// get_token takes in an OKTA username and password, and returns an access token for an application
#[tokio::main]
pub async fn get_token(username: String, password: String, config: OktaConfig) -> Result<String, String> {

    // Get config from well-known
    let res = reqwest::get(format!("{}/oauth2/default/.well-known/openid-configuration", config.url)).await;
    let res = res.expect("Error getting Discovery").json::<HashMap<String, serde_json::Value>>().await;
    let res = res.expect("Error parsing JSON");

    let authorization_endpoint = res["authorization_endpoint"].as_str().expect("Missing authorization_endpoint");
    let token_endpoint = res["token_endpoint"].as_str().expect("Missing token_endpoint");

    // Create a code verifier and code challenge
    let code_verifier = pkce::code_verifier(128);
    let code_challenge = pkce::code_challenge(&code_verifier);
    let code_verifier_string = String::from_utf8(code_verifier).expect("Couldn't convert verifier to string");

    // Use Username and Password to get a Session Token
    let client = reqwest::Client::new();

    // Post to /authn with username and password
    let req = client.post(format!("{}/api/v1/authn", config.url)).json(&serde_json::json!({
        "username": username.to_string(),
        "password": password.to_string()
    })).send().await;

    // Get Session Token from the JSON response
    let res = req.expect("Error getting Session Token").json::<HashMap<String, serde_json::Value>>().await;
    let res = res.expect("Invalid JSON");

    let session_token = match res.get("sessionToken") {
        Some(token) => Ok(token.as_str().expect("sessionToken was not a string as expected")),
        None => Err("sessionToken was missing, maybe the username and password are incorrect?")
    }?;

    // Use Session Token to get an Auth Code
    let params = [
        ("client_id", config.client_id.to_string()),
        ("response_type", "code".to_string()),
        ("code_challenge_method", "S256".to_string()),
        ("code_challenge", code_challenge),
        ("redirect_uri", config.redirect_uri.to_string()),
        ("scope", "openid email groups profile".to_string()),
        ("prompt", "none".to_string()),
        ("response_mode", "form_post".to_string()),
        ("state", "a".to_string()),
        ("nonce", "a".to_string()),
        ("sessionToken", session_token.to_string())
    ];
    let url = reqwest::Url::parse_with_params(authorization_endpoint, &params).expect("Failed to create URL");

    // Get Text Response to parse HTML for the code response
    let req = client.get(url).send().await;
    let text = req.expect("Error getting Code").text().await;
    let text = text.expect("Invalid Text");

    // Scrape code from <input name='code' value='....' />
    let dom = scraper::Html::parse_document(&text);
    let selector = scraper::Selector::parse(r#"input[name="code"]"#).unwrap();
    let code = dom.select(&selector).next().expect("Missing Input with code").value().attr("value").expect("Missing value on code");

    // Use Auth Code to get Access Token
    let form = [
        ("client_id", config.client_id.to_string()),
        ("code_verifier", code_verifier_string.to_string()),
        ("redirect_uri", config.redirect_uri.to_string()),
        ("grant_type", "authorization_code".to_string()),
        ("code", code.to_string())
    ];

    let req = client.post(token_endpoint).form(&form).send().await;
    let json = req.expect("Error getting Access Token").json::<HashMap<String, serde_json::Value>>().await;
    let json = json.expect("Invalid JSON");
    let access_token = json["access_token"].as_str().expect("Missing access token");

    // Return access token
    Ok(access_token.to_string())
}

