# auth-token

## Summary
A small CLI application and rust library, that allows you to get an auth token for use in development.

## Example
```bash
# Get an OKTA Access Token
$ auth-token  okta-access-token
             --client-id XXXXyyyy
             --base-url https://myapp.okta.com/ 
             --authorization-server-id abc123
             --login-redirect-url http://myapp/callback 
             --username my.user 
             --copy-to-clipboard

🎉 auth-token - A CLI tool to get an auth token for use in development.
Password? (hidden) 

🔐 Getting Access Token for my.user

✅ Token Copied To Clipboard

eyJra....
```

## Usage
```bash
auth-token command --flags

# Commands
okta-access-token

# Flags
--base-url https://myapp.okta.com/
--client-id XXXXyyyy
--authorization-server-id abc123
--login-redirect-url http://myapp/callback
--scopes 'openid profile email'
--username my.user
--password pa$sw0rd
--copy-to-clipboard
--print-token-json
```
