/// Call the OAuth authorize endpoint

use crate::okta::OktaClient;
use std::string::String;
use rand::Rng;
use crate::okta::error::OktaClientError;

#[derive(Debug)]
pub enum CodeChallengeMethod {
    //Plain,      // Not Implemented
    S256
}

impl std::fmt::Display for CodeChallengeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            //CodeChallengeMethod::Plain => f.write_str("plain"),
            CodeChallengeMethod::S256 => f.write_str("s256"),
        }
    }
}

#[derive(Debug)]
pub enum ResponseType {
    Code,
    //Token,      // Not Implemented
    //IdToken,    // Not Implemented
    //None        // Not Implemented
}

impl std::fmt::Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResponseType::Code => f.write_str("code"),
            //ResponseType::Token => f.write_str("token"),
            //ResponseType::IdToken => f.write_str("id_token"),
            //ResponseType::None => f.write_str("none"),
        }
    }
}

#[derive(Debug)]
pub enum ResponseMode {
    //Query,      // Not Implemented
    //Fragment,   // Not Implemented
    FormPost,
    //WebMessage  // Not Implemented
}

impl std::fmt::Display for ResponseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            //ResponseMode::Query => f.write_str("query"),
            //ResponseMode::Fragment => f.write_str("fragment"),
            ResponseMode::FormPost => f.write_str("form_post"),
            //ResponseMode::WebMessage => f.write_str("web_message"),
        }
    }
}

#[derive(Debug)]
pub enum Prompt {
    None,
}

impl std::fmt::Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Prompt::None => f.write_str("none"),
        }
    }
}


#[derive(Debug)]
pub struct OktaAuthorizeRequest {
    pub client_id: String,
    pub response_type: ResponseType,
    pub code_challenge_method: CodeChallengeMethod,
    pub code_challenge: String,
    pub redirect_uri: String,
    pub scope: String,
    pub prompt: Prompt,
    pub response_mode: ResponseMode,
    pub state: String,
    pub nonce: String,
    pub session_token: String,
}

impl OktaAuthorizeRequest {
    pub fn as_params(&self) -> Vec<(&str, String)> {

        vec![
            ("client_id", self.client_id.to_owned()),
            ("response_type", self.response_type.to_string().to_owned()),
            ("code_challenge_method", self.code_challenge_method.to_string().to_owned()),
            ("code_challenge", self.code_challenge.to_owned()),
            ("redirect_uri", self.redirect_uri.to_owned()),
            ("scope", self.scope.to_owned()),
            ("prompt", self.prompt.to_string().to_owned()),
            ("response_mode", self.response_mode.to_string().to_owned()),
            ("state", self.state.to_owned()),
            ("sessionToken", self.session_token.to_owned()),
            ("nonce", self.nonce.to_owned()),
        ]
    }
}

impl OktaClient {

    /// Use a Session Token to get an auth code
    pub async fn do_oauth_authorize(&self, session_token: String) -> Result<String, OktaClientError> {

        let request = OktaAuthorizeRequest {
            client_id: self.client_id.to_owned(),
            response_type: ResponseType::Code,
            code_challenge_method: CodeChallengeMethod::S256,
            code_challenge: self.pkce.code_challenge.to_owned(),
            redirect_uri: self.login_redirect_url.to_owned(),
            scope: self.scopes.to_owned(),
            prompt: Prompt::None,
            response_mode: ResponseMode::FormPost,
            state: random_string().to_owned(),
            nonce: random_string().to_owned(),
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
}

fn random_string() -> String {
     rand::thread_rng()
         .sample_iter(&rand::distributions::Alphanumeric)
         .take(8)
         .map(char::from)
         .collect()
}