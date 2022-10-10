use auth_token::okta::{OktaClientError, OktaClient};
use clap::{App, Arg};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use colored::*; // TODO narrow scope

fn main() {
    let matches = App::new("auth-token")
        .version("0.2.0")
        .author("Kye Lewis <kye.lewis@exogee.com>")
        .about("A CLI tool to get an access token for use in development.")
         .subcommand(App::new("okta-access-token").about("Returns an OKTA access token")
        .arg(
            Arg::new("base-url")
                .long("base-url")
                .value_name("base-url")
                .about("Base URL of the OKTA Tenant (ie. https://myapp.okta.com/)")
                .required(true),
        )
        .arg(
            Arg::new("client-id")
                .value_name("client-id")
                .long("client-id")
                .about("The OKTA Client ID associated with the app")
                .required(true),
        )
        .arg(
            Arg::new("authorization-server-id")
                .value_name("authorization-server-id")
                .long("authorization-server-id")
                .about(
                    "If using a custom Authorization Server, the ID for that authorization server",
                )
                .default_value("default"),
        )
        .arg(
            Arg::new("copy-to-clipboard")
                .long("copy-to-clipboard")
                .value_name("copy-to-clipboard")
                .takes_value(false)
                .about("Copy the result to the system clipboard"),
        )
        .arg(
            Arg::new("print-token-json")
                .long("print-token-json")
                .value_name("print-token-json")
                .takes_value(false)
                .about("Print the JSON of the token to stdout instead of the token itself"),
        )
        .arg(
            Arg::new("login-redirect-url")
                .long("login-redirect-url")
                .value_name("login-redirect-url")
                .about("OKTA Login Redirect URL associated with the app")
                .required(true),
        )
        .arg(
            Arg::new("scopes")
                .long("scopes")
                .value_name("scopes")
                .about("The scope(s) to request (ie. openid profile email)")
                .default_value("openid profile email"),
        )
        .arg(
            Arg::new("username")
                .long("username")
                .value_name("username")
                .about("OKTA username (optional, prompted on CLI if omitted)")
                .required(false),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .value_name("password")
                .about("OKTA password (optional, prompted on CLI if omitted)")
                .required(false),
         ))
        .get_matches();

    eprintln!("🎉 auth-token - A CLI tool to get an access token for use in development.");

    match matches.subcommand() {
        Some(("okta-access-token", args)) => {

    // Read Base URL, Redirect URL and Client ID from flags.
    let url = args.value_of("base-url").unwrap().to_owned();
    let login_redirect_url = args.value_of("login-redirect-url").unwrap().to_owned();
    let client_id = args.value_of("client-id").unwrap().to_owned();
    let authorization_server_id = args
        .value_of("authorization-server-id")
        .unwrap()
        .to_owned();
    let scopes = args.value_of("scopes").unwrap().to_owned();
    let copy_to_clipboard = args.is_present("copy-to-clipboard");
    let print_token_json = args.is_present("print-token-json");

    // Read Username and Password from flags, if provided, otherwise read from CLI.
    let username = args
        .value_of("username")
        .map(|s| s.to_owned())
        .unwrap_or_else(|| read_input("Username? (hidden) ".to_owned()));

    let password = args
        .value_of("password")
        .map(|s| s.to_owned())
        .unwrap_or_else(|| read_input("Password? (hidden) ".to_owned()));

    get_access_token(
            url,
            login_redirect_url,
            client_id,
            authorization_server_id,
            username,
            password,
            scopes,
            copy_to_clipboard,
            print_token_json,
        )

        },
        _ => {
            println!("Run with --help for usage.")
        },
    }

}

fn get_access_token(
    url: String,
    login_redirect_url: String,
    client_id: String,
    authorization_server_id: String,
    username: String,
    password: String,
    scopes: String,
    copy_to_clipboard: bool,
    print_token_json: bool,
) -> () {
    eprintln!(
        "🔐 Getting Access Token for {}",
        username.to_owned().underline()
    );

    let client = OktaClient::new(
        username.to_owned(),
        password.to_owned(),
        client_id.to_owned(),
        authorization_server_id.to_owned(),
        login_redirect_url.to_owned(),
        url.to_owned(),
        scopes.to_owned(),
    );

    match client {
        Ok(client) => {
            let token = client.get_access_token().unwrap_or_else(|e| show_error(e));

            if !print_token_json {
                // Print token to stdout
                println!("{}", token);

                if copy_to_clipboard {
                    eprintln!(
                        "✅  {}",
                        "Access Token Copied To Clipboard\n".green().bold()
                    );
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(token.to_owned()).unwrap();
                }
            } else {
                // Parse token
                if let Some(token_section) = token.split(".").nth(1) {
                    if let Ok(decoded_token_section) = base64::decode(token_section) {
                        if let Ok(decoded_token_section_string) =
                            std::str::from_utf8(&decoded_token_section)
                        {
                            println!("{}", decoded_token_section_string);

                            if copy_to_clipboard {
                                eprintln!(
                                    "✅  {}",
                                    "Access Token JSON Copied To Clipboard\n".green().bold()
                                );
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                ctx.set_contents(decoded_token_section_string.to_owned())
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            show_error(e);
        }
    }
}

fn read_input(message: String) -> String {
    rpassword::prompt_password_stderr(&message).unwrap_or("".to_owned())
}

fn show_error(error: OktaClientError) -> String {
    eprintln!("😔 {} {}", "Error:".red().bold(), error);
    std::process::exit(1);
}
