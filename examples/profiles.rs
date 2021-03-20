#![deny(missing_debug_implementations, missing_docs)]

//! Reads credentials for a given profile from file, or prompts the user if they
//! don't exist.
//!
//! ```text,ignore
//!                _         _
//!  ___ ___ ___ _| |___ ___| |_
//! |  _|  _| -_| . | -_|   |  _|
//! |___|_| |___|___|___|_|_|_|
//! ```

use std::{env, env::Args, ffi::OsStr, fmt::Write, path::PathBuf};

use credent::{
    cli::CredentialsCliReader,
    fs::{model::AppName, CredentialsFileLoader, CredentialsFileStorer},
    model::{Credentials, Password},
};

use demo_styles::{Colours, Logo, Prompt};

mod demo_styles;

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

type CredentialsFile = credent::fs::CredentialsFile<Credentials>;
type Profile = credent::model::Profile<Credentials>;

#[cfg(feature = "backend-smol")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Logo::ascii_coloured());

    let result = smol::block_on(async {
        let profile_name = get_profile_name()?;
        let credentials = match existing_credentials(&profile_name).await? {
            Some(credentials) => credentials,
            None => prompt_and_save_credentials(profile_name.clone()).await?,
        };
        println!("");

        output_profile_name(&profile_name);
        output_credentials(&credentials);
        output_password(&credentials.password);

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    });

    if let Err(e) = result {
        eprintln!(
            "{error} {message}",
            error = Colours::error_label().apply("Error:"),
            message = e
        );
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(feature = "backend-tokio")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;

    println!("{}", Logo::ascii_coloured());

    let result = rt.block_on(async {
        let profile_name = get_profile_name()?;
        let credentials = match existing_credentials(&profile_name).await? {
            Some(credentials) => credentials,
            None => prompt_and_save_credentials(profile_name.clone()).await?,
        };
        println!("");

        output_profile_name(&profile_name);
        output_credentials(&credentials);
        output_password(&credentials.password);

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    });

    if let Err(e) = result {
        eprintln!(
            "{error} {message}",
            error = Colours::error_label().apply("Error:"),
            message = e
        );
        std::process::exit(1);
    }
    Ok(())
}

fn get_profile_name() -> Result<String, String> {
    let mut args = env::args();

    let full_path = args.next().map(PathBuf::from);
    let exe_name = full_path
        .as_ref()
        .map(|full_path| full_path.file_name())
        .flatten()
        .map(OsStr::to_str)
        .flatten()
        .unwrap_or("profiles");

    match args.next().as_deref() {
        None => Ok(Profile::DEFAULT_NAME.to_string()),
        Some("--profile") => next_arg_as_profile(exe_name, args.next()),
        Some(unknown_arg) => {
            let message =
                handle_unknown_arg(exe_name, unknown_arg, args).map_err(|e| format!("{}", e))?;

            Err(message)
        }
    }
}

fn next_arg_as_profile(exe_name: &str, next_arg: Option<String>) -> Result<String, String> {
    if let Some(profile_name) = next_arg {
        Ok(profile_name)
    } else {
        let message = format!(
            "\
            Profile name must be specified.\n\
            \n\
            {arrow}{exe_name} --profile {profile_placeholder}\n\
            {indent}{highlight:>pad$}\n\
            ",
            arrow = Colours::prompt_label().apply("> "),
            exe_name = exe_name,
            profile_placeholder = Colours::error_label().apply(".."),
            indent = "  ",
            highlight = Colours::error_label().apply("^^^^^^^^^^^^"),
            pad = exe_name.len() + " --profile ".len() + "^^".len()
        );
        Err(message)
    }
}

fn handle_unknown_arg(
    exe_name: &str,
    unknown_arg: &str,
    mut args: Args,
) -> Result<String, std::fmt::Error> {
    let highlight_str = "^".repeat(unknown_arg.len());
    let mut message = String::with_capacity(512);
    let buffer = &mut message;
    writeln!(buffer, "Invalid argument in command line.")?;
    writeln!(buffer)?;

    write!(
        buffer,
        "{arrow}{exe_name} {unknown_arg}",
        arrow = Colours::prompt_label().apply("> "),
        exe_name = exe_name,
        unknown_arg = unknown_arg
    )?;
    args.try_for_each(|arg| write!(buffer, " {}", arg))?;
    writeln!(buffer)?;

    writeln!(
        buffer,
        "{indent}{highlight:>pad$}",
        indent = "  ",
        highlight = Colours::error_label().apply(highlight_str),
        pad = exe_name.len() + 1 + unknown_arg.len()
    )?;

    Ok(message)
}

async fn existing_credentials(
    profile_name: &str,
) -> Result<Option<Credentials>, Box<dyn std::error::Error>> {
    let profile = CredentialsFileLoader::load_profile(CREDENT, profile_name).await?;
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

async fn prompt_and_save_credentials(
    profile_name: String,
) -> Result<Credentials, Box<dyn std::error::Error>> {
    let credentials_cli_reader = CredentialsCliReader {
        username_prompt: Prompt::username(),
        password_prompt: Prompt::password(),
    };

    let credentials = credentials_cli_reader.prompt_from_tty().await?;
    println!("");

    let profile = Profile::new(profile_name, credentials);
    CredentialsFileStorer::store(CREDENT, &profile).await?;

    println!(
        "{note} Stored credentials in `{path}`.",
        note = Colours::informative_label().apply("Note:"),
        path = Colours::informative_value().apply(CredentialsFile::path(CREDENT)?.display()),
    );

    Ok(profile.credentials)
}

fn output_profile_name(profile_name: &str) {
    println!(
        "{l}{profile}{r}",
        l = Colours::prompt_label().apply("["),
        profile = Colours::prompt_label().apply(profile_name),
        r = Colours::prompt_label().apply("]"),
    );
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
