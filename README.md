# get-token - A CLI to get an auut token

## Summary
A small command-line application, written in rust, that allows you to get an OKTA token for use in an app.

## Screenshot
<img src="screenshot.png?raw=true" height="200">

## Binary Releases
See [Github Releases](https://github.com/TechInSite/okta-token-cli/releases)

## Example
```bash
# Get an OKTA Access Toke
$ get-token  okta-access-token
             --client-id XXXXyyyy
             --base-url https://myapp.okta.com/ 
             --authorization-server-id abc123
             --login-redirect-url http://myapp/callback 
             --username my.user 
             --copy-to-clipboard

🎉 get-token - A CLI tool to get an auth token
Password? (hidden) 

🔐 Getting Access Token for my.user
✅ OKTA Token Copied To Clipboard

eyJra....
```

## Usage
```bash
tako [--flags] [command]

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

## Setting up on Mac
Download the release from [Github Releases](https://github.com/TechInSite/okta-token-cli/releases) and copy to your home directory.

Run the file once by using right click / option click -> run, to approve the binary through gatekeeper.

Add a tako command to the end of your `~/.zshrc` file:
```bash
alias token="get-toke --client-id=XXXXyyyy --base-url=https://myapp.okta.com/ --login-redirect-url=http://myapp/callback --scopes='openid profile email groups' --username=my.user --copy-to-clipboard get-access-token"
```

Open a new terminal, and run `token`!

## Limitations
- Binary Releases are not notarized yet, so we can't create an install script.
- Only basic auth (user/password) is implemented.
- Only the following modes are impemented:
  - `code_challenge_method: S256` 
  - `response_type: code`
  - `response_mode: form_post`
  - `prompt: none`
  - `grant_type: authorization_code`
- Error codes are not read from some endpoints, instead a generic error is returned.

## Build Source
`rustup` must already be installed - https://www.rust-lang.org/tools/install

```bash
# Build for your own platform
make

# Install cross-compilation chains if not already done.
# mingw-w64 is also required to build a windows target.
make install-toolchains 

# Build for other platforms
make build-mac-aarch64
make build-mac
make build-win
make build-linux
make build-linux-musl
```

## Codesign and Notarize
We are using `gon` to assist in codesign and notarization. Run the following command from the project root directory. 

```bash
BUNDLE_ID=your.bundle.id AC_USERNAME=apple.connect.username AC_PASSWORD=app.specific.password gon gon_config.json
```
