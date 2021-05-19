# OKTA Token CLI
Get an OKTA token for an app.

## Build
Make sure you have the `x86_64-apple-darwin` target to cross-compile if you are building on Apple Silicon.

```bash
rustup target add x86_64-apple-darwin
```

Then run the build script.

```bash
$ ./build.sh
```

## Usage
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

```bash
# Without Password
$ OKTA_CLIENT_ID='xxxxyyyy' \
   OKTA_URL='https://demo.okta.com/' \
   OKTA_LOGIN_REDIRECT_URI='https://demo.site/login' \
   ./okta-token user.name

🕶️ OKTA Token Tool
Password? 

🔐 Getting Token for kye.lewis : ********
✅ OKTA Token Copied To Clipboard

eyJra....
```