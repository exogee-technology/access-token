pub struct PKCE {
    pub code_verifier: String,
    pub code_challenge: String,
}

impl PKCE {
    /// Create a new Code Verifier
    pub fn new() -> Self {
        let code_verifier = pkce::code_verifier(128);
        PKCE {
            code_verifier: String::from_utf8(code_verifier.clone())
                .expect("Couldn't convert from vec to string"),
            code_challenge: pkce::code_challenge(&code_verifier),
        }
    }
}
