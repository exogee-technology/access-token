mod okta;

extern crate colored;
use colored::*;

extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {

    eprintln!("{}", "\n🎉 OKTA Token Tool".bold());

    let client_id = std::env::var("OKTA_CLIENT_ID").unwrap_or_else(|_| show_error(String::from("Missing OKTA_CLIENT_ID Environment Variable")));
    let url = std::env::var("OKTA_URL").unwrap_or_else(|_| show_error(String::from("Missing OKTA_URL Environment Variable")));
    let redirect_uri = std::env::var("OKTA_LOGIN_REDIRECT_URL").unwrap_or_else(|_| show_error(String::from("Missing OKTA_LOGIN_REDIRECT_URL Environment Variable")));

    let username = std::env::args().nth(1).unwrap_or_else(|| read_input(String::from("Username? (hidden) ")));
    let password = std::env::args().nth(2).unwrap_or_else(|| read_input(String::from("Password? (hidden) ")));
    let masked_password = String::from_utf8(vec![b'*'; password.len()]).unwrap_or(String::from("*****"));


    eprintln!("\n🔐 Getting Token for {} : {}", String::from(&username).underline(), String::from(&masked_password).underline());

    let token = okta::get_token(
        String::from(&username),
        String::from(&password),
        okta::OktaConfig {
            client_id: String::from(client_id),
            url: String::from(url),
            redirect_uri: String::from(redirect_uri)
        }
    );

    let token = token.unwrap_or_else(|error| show_error(String::from(error)));

    eprintln!("✅  {}", "OKTA Token Copied To Clipboard\n".green().bold());
    println!("{}", token);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(token.to_owned()).unwrap();

}

fn read_input(message: String) -> String {
    rpassword::prompt_password_stderr(&message).unwrap_or(String::from(""))

}

fn show_error(error: String) -> String {
    eprintln!("😔 {} {}", "Error:".red().bold(), error);
    std::process::exit(1);
}