#![deny(missing_debug_implementations, missing_docs)]

//! Control Credent from the command line.
//!
//! ```text,ignore
//!                _         _
//!  ___ ___ ___ _| |___ ___| |_
//! |  _|  _| -_| . | -_|   |  _|
//! |___|_| |___|___|___|_|_|_|
//! ```

use credent::{
    cli::CredentialsCliReader,
    fs::{AppName, CredentialsFile, CredentialsFileLoader, CredentialsFileStorer},
    model::{Credentials, Password, Profile},
};

use demo_styles::{Colours, Logo, Prompt};

mod demo_styles;

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Logo::ascii_coloured());

    smol::run(async {
        let credentials_cli_reader = CredentialsCliReader {
            username_prompt: Prompt::username(),
            password_prompt: Prompt::password(),
        };

        let credentials_result = CredentialsFileLoader::load(CREDENT).await;
        let profile = if let Some(credentials_result) = credentials_result {
            println!(
                "{note} Read credentials from `{path}`.",
                note = Colours::informative_label().apply("Note:"),
                path =
                    Colours::informative_value().apply(CredentialsFile::path(CREDENT)?.display()),
            );

            credentials_result?
        } else {
            let credentials = credentials_cli_reader.prompt_from_tty().await?;
            let profile = Profile::new_default(credentials);
            CredentialsFileStorer::store(CREDENT, &profile).await?;

            println!("");
            println!(
                "{note} Stored credentials in `{path}`.",
                note = Colours::informative_label().apply("Note:"),
                path =
                    Colours::informative_value().apply(CredentialsFile::path(CREDENT)?.display()),
            );

            profile
        };
        println!("");

        output_credentials(&profile.credentials);
        println!("");
        output_password(&profile.credentials.password);

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}

fn output_credentials(credentials: &Credentials) {
    println!("{}", Colours::output_label().apply("credentials:"),);
    println!(
        "  {hint:-12}: {value}",
        hint = Colours::output_hint().apply("to_string()"),
        value = credentials
    );
    println!(
        "  {hint:-12}: {value:?}",
        hint = Colours::output_hint().apply("debug"),
        value = credentials
    );
}

fn output_password(password: &Password) {
    println!("{}", Colours::output_label().apply("password:"),);
    println!(
        "  {hint:-12}: {value}",
        hint = Colours::output_hint().apply("to_string()"),
        value = password
    );
    println!(
        "  {hint:-12}: {value}",
        hint = Colours::output_hint().apply("encoded()"),
        value = password.encoded()
    );
    println!(
        "  {hint:-12}: {value}",
        hint = Colours::output_hint().apply("plain_text()"),
        value = password.plain_text()
    );
}
