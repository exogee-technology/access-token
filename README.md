<img src="logo.png?raw=true" width="200">

# tako: An OKTA Token CLI
Get an OKTA token for an app.

## Binary Releases
See github releases

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

## Usage
- Provide `OKTA_CLIENT_ID`, `OKTA_URL` and `OKTA_LOGIN_REDIRECT_URI` environment variables.
- If a username or password is not supplied, you will be prompted for the missing detail at runtime.
- For quick usage, create an alias in your `.bashrc` or `.zshrc`.
- If succesful, the OKTA token is displayed, and also copied to your clipboard, ready to use.
```bash
# Get an Access Token with provided username
$ tako  --client-id=XXXXyyyy
        --base-url=https://myapp.okta.com/ 
        --login-redirect-url=http://myapp/callback 
        --scopes='openid profile email groups' 
        --username=my.user 
        get-access-token


🎉 tako - An OKTA CLI Tool
Password? (hidden) 

🔐 Getting Access Token for kye.lewis
✅ OKTA Token Copied To Clipboard

eyJra....
```

## Setting up on Mac
Download the release from [Github Releases](https://github.com/TechInSite/okta-token-cli/releases) and copy to your home directory.

Run the file once by using right click / option click -> run, to approve the binary through gatekeeper.

Add a tako command to the end of your `~/.zshrc` file:
```bash
alias token="tako --client-id=XXXXyyyy --base-url=https://myapp.okta.com/ --login-redirect-url=http://myapp/callback --scopes='openid profile email groups' --username=my.user get-access-token"
```

Open a new terminal, and run `token`!
