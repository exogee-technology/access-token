use error::OktaClientError;

mod authn;
mod authorize;
pub mod error;
mod openid;
mod pkce;
mod token;

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
    pub fn new(
        username: String,
        password: String,
        client_id: String,
        authorization_server_id: String,
        login_redirect_url: String,
        base_url: String,
        scopes: String,
    ) -> Result<Self, OktaClientError> {
        let openid_config =
            openid::get_openid_config(base_url.to_owned(), authorization_server_id.to_owned())?;

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

    /// Get an access token for a specific OKTA tenant and client/app
    #[tokio::main]
    pub async fn get_access_token(&self) -> Result<String, OktaClientError> {
        // Get Session token from /authn
        let okta_session = self.do_okta_authn().await?;

        // Get Auth Code from /authorization
        let auth_code = self
            .do_oauth_authorize(
                okta_session
                    .session_token
                    .expect("Missing Session Token")
                    .to_owned(),
            )
            .await?;

        // Get Access Token from /token
        let token = self.do_oauth_token(auth_code.to_owned()).await?;

        Ok(token)
    }
}
