mod okta;

use colored::*;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use clap::{App,Arg};

fn main() {

    let matches = App::new("tako")
        .version("1.0")
        .author("Kye Lewis <kye.lewis@techin.site>")
        .about("An OKTA CLI Tool")
        .arg(Arg::new("base-url")
            .long("base-url")
            .value_name("base-url")
            .about("Base URL of the OKTA Tenant (ie. https://myapp.okta.com/)")
            .required(true))
        .arg(Arg::new("client-id")
            .value_name("client-id")
            .long("client-id")
            .about("The OKTA Client ID associated with the app")
            .required(true))
        .arg(Arg::new("copy-to-clipboard")
            .long("copy-to-clipboard")
            .value_name("copy-to-clipboard")
            .takes_value(false)
            .about("Copy the result to the system clipboard"))
        .arg(Arg::new("login-redirect-url")
            .long("login-redirect-url")
            .value_name("login-redirect-url")
            .about("OKTA Login Redirect URL associated with the app")
            .required(true))
        .arg(Arg::new("scopes")
            .long("scopes")
            .value_name("scopes")
            .default_missing_value("openid profile email")
            .about("The scope(s) to request (ie. openid profile email)")
            .required(true))
        .arg(Arg::new("username")
            .long("username")
            .value_name("username")
            .about("OKTA username (optional, prompted on CLI if omitted)")
            .required(false))
        .arg(Arg::new("password")
            .long("password")
            .value_name("password")
            .about("OKTA password (optional, prompted on CLI if omitted)")
            .required(false))
        .subcommand(App::new("get-access-token")
            .about("Returns a client access token"))
        .get_matches();

    eprintln!("🎉 tako - An OKTA CLI Tool");

    // Read Base URL, Redirect URL and Client ID from flags.
    let url = matches.value_of("base-url").unwrap().to_owned();
    let login_redirect_url = matches.value_of("login-redirect-url").unwrap().to_owned();
    let client_id = matches.value_of("client-id").unwrap().to_owned();
    let scopes = matches.value_of("scopes").unwrap().to_owned();
    let copy_to_clipboard = matches.is_present("copy-to-clipboard");

    // Read Username and Password from flags, if provided, otherwise read from CLI.
    let username = matches.value_of("username")
        .map(|s| s.to_owned())
        .unwrap_or_else(|| read_input("Username? (hidden) ".to_owned()));

    let password = matches.value_of("password")
        .map(|s| s.to_owned())
        .unwrap_or_else(|| read_input("Password? (hidden) ".to_owned()));

    match matches.subcommand() {
        Some(("get-access-token", _)) => get_access_token(url, login_redirect_url, client_id, username, password, scopes, copy_to_clipboard),
        _ => {},
    }

}

fn get_access_token(url: String, login_redirect_url: String, client_id: String, username: String, password: String, scopes: String, copy_to_clipboard: bool) -> () {

    eprintln!("\n🔐 Getting Access Token for {}", username.to_owned().underline());

    let client = okta::OktaClient::new(
        username.to_owned(),
        password.to_owned(),
        client_id.to_owned(),
        login_redirect_url.to_owned(),
        url.to_owned(),
        scopes.to_owned());

    match client {
        Ok(client) => {
            let token = client.get_access_token().unwrap_or_else(|e| show_error(e));

            // Print token to stdout
            println!("{}", token);

            if copy_to_clipboard {
                eprintln!("\n✅  {}", "OKTA Token Copied To Clipboard\n".green().bold());
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(token.to_owned()).unwrap();
            }
        },
        Err(e) => {
            show_error(e);
        }
    }

}

fn read_input(message: String) -> String {
    rpassword::prompt_password_stderr(&message).unwrap_or("".to_owned())

}

fn show_error(error: String) -> String {
    eprintln!("😔 {} {}", "Error:".red().bold(), error);
    std::process::exit(1);
}