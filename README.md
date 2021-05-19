# OKTA Token CLI
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
# With Username and Password
$ OKTA_CLIENT_ID='xxxxyyyy' \
   OKTA_URL='https://demo.okta.com/' \
   OKTA_LOGIN_REDIRECT_URI='https://demo.site/login' \
   ./okta-token user.name passw0rd

🕶️ OKTA Token Tool

🔐 Getting Token for kye.lewis : *****
✅ OKTA Token Copied To Clipboard

eyJra....
```