use std::io;

use smol::{io::Error, prelude::AsyncWriteExt, Unblock};

use credent_auth_model::{Credentials, Password, Username};

/// Reads `Credentials` from the command line.
#[derive(Debug)]
pub struct CredentialsCliReader;

impl CredentialsCliReader {
    /// Reads the username and password from the terminal.
    pub async fn read_from_tty() -> Result<Credentials, Error> {
        let username = Self::read_username().await?;
        let password = Self::read_password().await?;

        Ok(Credentials { username, password })
    }

    /// Reads the username from the terminal.
    pub async fn read_username() -> Result<Username, Error> {
        let prompt = "Username: ";
        let mut stderr = Unblock::new(io::stderr());
        stderr.write_all(prompt.as_bytes()).await?;
        stderr.flush().await?;

        let username = smol::unblock! {
            let mut username = String::new();
            io::stdin().read_line(&mut username).map(|_| Username(username.trim().to_string()))
        }?;

        Ok(username)
    }

    /// Reads the password from the terminal.
    pub async fn read_password() -> Result<Password, Error> {
        let prompt = "Password (input will be hidden): ";
        let mut stderr = Unblock::new(io::stderr());
        stderr.write_all(prompt.as_bytes()).await?;
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
