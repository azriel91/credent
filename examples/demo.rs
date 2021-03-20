#![deny(missing_debug_implementations, missing_docs)]

//! Reads credentials from file, or prompts the user if they don't exist.
//!
//! ```text,ignore
//!                _         _
//!  ___ ___ ___ _| |___ ___| |_
//! |  _|  _| -_| . | -_|   |  _|
//! |___|_| |___|___|___|_|_|_|
//! ```

use credent::{
    cli::CredentialsCliReader,
    fs::{model::AppName, CredentialsFileLoader, CredentialsFileStorer},
    model::{Credentials, Password, Profile},
};

use demo_styles::{Colours, Logo, Prompt};

mod demo_styles;

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

type CredentialsFile = credent::fs::CredentialsFile<Credentials>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Logo::ascii_coloured());

    smol::block_on(async {
        let credentials = match existing_credentials().await? {
            Some(credentials) => credentials,
            None => prompt_and_save_credentials().await?,
        };
        println!("");

        output_credentials(&credentials);
        output_password(&credentials.password);

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}

async fn existing_credentials() -> Result<Option<Credentials>, Box<dyn std::error::Error>> {
    let profile = CredentialsFileLoader::load(CREDENT).await?;
    if profile.is_some() {
        println!(
            "{note} Read existing credentials from `{path}`.",
            note = Colours::informative_label().apply("Note:"),
            path = Colours::informative_value().apply(CredentialsFile::path(CREDENT)?.display()),
        );

        Ok(profile.map(|profile| profile.credentials))
    } else {
        Ok(None)
    }
}

async fn prompt_and_save_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    let credentials_cli_reader = CredentialsCliReader {
        username_prompt: Prompt::username(),
        password_prompt: Prompt::password(),
    };

    let credentials = credentials_cli_reader.prompt_from_tty().await?;
    println!("");

    let profile = Profile::new_default(credentials);
    CredentialsFileStorer::store(CREDENT, &profile).await?;

    println!(
        "{note} Stored credentials in `{path}`.",
        note = Colours::informative_label().apply("Note:"),
        path = Colours::informative_value().apply(CredentialsFile::path(CREDENT)?.display()),
    );

    Ok(profile.credentials)
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
    println!("");
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
