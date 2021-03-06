use std::{fmt::Display, io};

use credent_cli_model::Error;
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

#[cfg(feature = "smol")]
use smol::{io::AsyncWriteExt, Unblock};

#[cfg(feature = "smol")]
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
        let prompt = self.username_prompt.to_string();
        let username = Self::prompt_plain_text(&prompt)
            .await
            .map(Username)
            .map_err(|e| {
                if let Error::PlainTextRead(error) = e {
                    Error::UsernameRead(error)
                } else {
                    e
                }
            })?;

        Ok(username)
    }

    /// Reads the password from the terminal.
    pub async fn prompt_password(&self) -> Result<Password, Error> {
        let prompt = self.password_prompt.to_string();
        let password = Self::prompt_secret(&prompt)
            .await
            .map(Password::new)
            .map_err(|e| {
                if let Error::SecretRead(error) = e {
                    Error::PasswordRead(error)
                } else {
                    e
                }
            })?;

        Ok(password)
    }

    /// Reads a plain text value from the terminal.
    pub async fn prompt_plain_text(prompt: &str) -> Result<String, Error> {
        let mut stderr = Unblock::new(io::stderr());
        stderr.write_all(prompt.as_bytes()).await.map_err(|error| {
            let prompt = prompt.to_string();
            Error::PromptWrite { prompt, error }
        })?;
        stderr.flush().await.map_err(Error::StdErrFlush)?;

        let value = smol::unblock(|| {
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .map(|_| value.trim().to_string())
        })
        .await
        .map_err(Error::PlainTextRead)?;

        Ok(value)
    }

    /// Reads a secret value from the terminal.
    pub async fn prompt_secret(prompt: &str) -> Result<String, Error> {
        let mut stderr = Unblock::new(io::stderr());
        stderr.write_all(prompt.as_bytes()).await.map_err(|error| {
            let prompt = prompt.to_string();
            Error::PromptWrite { prompt, error }
        })?;
        stderr.flush().await.map_err(Error::StdErrFlush)?;

        // Read secret value on a separate thread.
        let secret =
            smol::unblock(|| rpassword::read_password_from_tty(None).map_err(Error::SecretRead))
                .await?;

        Ok(secret)
    }
}

#[cfg(feature = "tokio")]
use tokio::io::AsyncWriteExt;

#[cfg(feature = "tokio")]
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
        let prompt = self.username_prompt.to_string();
        let username = Self::prompt_plain_text(&prompt)
            .await
            .map(Username)
            .map_err(|e| {
                if let Error::PlainTextRead(error) = e {
                    Error::UsernameRead(error)
                } else {
                    e
                }
            })?;

        Ok(username)
    }

    /// Reads the password from the terminal.
    pub async fn prompt_password(&self) -> Result<Password, Error> {
        let prompt = self.password_prompt.to_string();
        let password = Self::prompt_secret(&prompt)
            .await
            .map(Password::new)
            .map_err(|e| {
                if let Error::SecretRead(error) = e {
                    Error::PasswordRead(error)
                } else {
                    e
                }
            })?;

        Ok(password)
    }

    /// Reads a plain text value from the terminal.
    pub async fn prompt_plain_text(prompt: &str) -> Result<String, Error> {
        let mut stderr = tokio::io::stderr();
        stderr.write_all(prompt.as_bytes()).await.map_err(|error| {
            let prompt = prompt.to_string();
            Error::PromptWrite { prompt, error }
        })?;
        stderr.flush().await.map_err(Error::StdErrFlush)?;

        let value = tokio::task::spawn_blocking(|| {
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .map(|_| value.trim().to_string())
        })
        .await
        .map_err(Error::StdinReadJoin)?
        .map_err(Error::PlainTextRead)?;

        Ok(value)
    }

    /// Reads a secret value from the terminal.
    pub async fn prompt_secret(prompt: &str) -> Result<String, Error> {
        let mut stderr = tokio::io::stderr();
        stderr.write_all(prompt.as_bytes()).await.map_err(|error| {
            let prompt = prompt.to_string();
            Error::PromptWrite { prompt, error }
        })?;
        stderr.flush().await.map_err(Error::StdErrFlush)?;

        // Read secret value on a separate thread.
        let secret = tokio::task::spawn_blocking(|| {
            rpassword::read_password_from_tty(None).map_err(Error::SecretRead)
        })
        .await
        .map_err(Error::StdinReadJoin)??;

        Ok(secret)
    }
}
