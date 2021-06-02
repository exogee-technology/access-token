#[derive(Debug)]
pub enum OktaClientError {
    Unknown,
    General(String),
    Network(String),
    OktaAPI(String),
}

impl From<reqwest::Error> for OktaClientError {
    fn from(error: reqwest::Error) -> Self {
        OktaClientError::Network(error.to_string())
    }
}

impl From<String> for OktaClientError {
    fn from(error: String) -> Self {
        OktaClientError::General(error.to_owned())
    }
}

impl std::fmt::Display for OktaClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OktaClientError::Unknown => f.write_str("Unknown Error"),
            OktaClientError::General(e) => f.write_str(&format!("General Error: {}", e)),
            OktaClientError::Network(e) => f.write_str(&format!("Network Error: {}", e)),
            OktaClientError::OktaAPI(e) => f.write_str(&format!("OKTA API Error: {}", e)),
        }
    }
}

