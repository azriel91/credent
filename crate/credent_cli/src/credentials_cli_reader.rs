use std::{fmt::Display, io};

use smol::{io::Error, prelude::AsyncWriteExt, Unblock};

use credent_model::{Credentials, Password, Username};

const CREDENTIALS_CLI_READER_PLAIN: CredentialsCliReader<&str, &str> = CredentialsCliReader {
    username_prompt: "Username: ",
    password_prompt: "Password (input is hidden): ",
};

/// Reads `Credentials` from the command line.
#[derive(Debug)]
pub struct CredentialsCliReader<UsernamePrompt, PasswordPrompt> {
    /// Prompt text for the username.
    pub username_prompt: UsernamePrompt,
    /// Prompt text for the password.
    pub password_prompt: PasswordPrompt,
}

impl CredentialsCliReader<(), ()> {
    /// Reads the username and password from the terminal.
    pub async fn read_from_tty() -> Result<Credentials, Error> {
        let username = Self::read_username().await?;
        let password = Self::read_password().await?;

        Ok(Credentials { username, password })
    }

    /// Reads the username from the terminal.
    pub async fn read_username() -> Result<Username, Error> {
        CREDENTIALS_CLI_READER_PLAIN.prompt_username().await
    }

    /// Reads the password from the terminal.
    pub async fn read_password() -> Result<Password, Error> {
        CREDENTIALS_CLI_READER_PLAIN.prompt_password().await
    }
}

impl<UsernamePrompt, PasswordPrompt> CredentialsCliReader<UsernamePrompt, PasswordPrompt>
where
    UsernamePrompt: Display,
    PasswordPrompt: Display,
{
    /// Reads the username and password from the terminal.
    pub async fn prompt_from_tty(&self) -> Result<Credentials, Error> {
        let username = self.prompt_username().await?;
        let password = self.prompt_password().await?;

        Ok(Credentials { username, password })
    }

    /// Reads the username from the terminal.
    pub async fn prompt_username(&self) -> Result<Username, Error> {
        let mut stderr = Unblock::new(io::stderr());
        stderr
            .write_all(self.username_prompt.to_string().as_bytes())
            .await?;
        stderr.flush().await?;

        let username = smol::unblock! {
            let mut username = String::new();
            io::stdin().read_line(&mut username).map(|_| Username(username.trim().to_string()))
        }?;

        Ok(username)
    }

    /// Reads the password from the terminal.
    pub async fn prompt_password(&self) -> Result<Password, Error> {
        let mut stderr = Unblock::new(io::stderr());
        stderr
            .write_all(self.password_prompt.to_string().as_bytes())
            .await?;
        stderr.flush().await?;

        // Read password on a separate thread.
        let password = smol::unblock! {
            rpassword::read_password_from_tty(None)
                .map(Password::new)
                .expect("Failed to read password from user input.")
        };

        Ok(password)
    }
}
