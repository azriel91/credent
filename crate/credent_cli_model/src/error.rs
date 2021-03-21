use std::fmt;

#[cfg(all(feature = "smol", not(feature = "tokio")))]
type IoError = smol::io::Error;

#[cfg(all(not(feature = "smol"), feature = "tokio"))]
type IoError = tokio::io::Error;

#[cfg(all(not(feature = "smol"), not(feature = "tokio")))]
compile_error!(
    r#"`credent` needs either the "backend-smol" or "backend-tokio" feature to be enabled."#
);

#[cfg(all(feature = "smol", feature = "tokio"))]
compile_error!(
    r#"Only one of "backend-smol" or "backend-tokio" should be enabled for `credent`.
Maybe different crates are using different features?"#
);

/// Errors when using `credenti_cli`.
#[derive(Debug)]
pub enum Error {
    /// Failed to write prompt to stderr.
    PromptWrite {
        /// Prompt to be written.
        prompt: String,
        /// Underlying error.
        error: IoError,
    },
    /// Failed to flush stderr.
    StdErrFlush(IoError),
    /// Failed to read username.
    UsernameRead(std::io::Error),
    /// Failed to read password.
    PasswordRead(std::io::Error),
    /// Failed to read a plain text value from stdin.
    PlainTextRead(std::io::Error),
    /// Failed to read a secret value from stdin.
    SecretRead(std::io::Error),

    /// Tokio blocking task join error.
    #[cfg(feature = "tokio")]
    StdinReadJoin(tokio::task::JoinError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PromptWrite { prompt, .. } => {
                write!(f, "Failed to write prompt to stderr. Prompt: `{}`", prompt)
            }
            Self::StdErrFlush(..) => write!(f, "Failed to flush `stderr`."),
            Self::UsernameRead(..) => write!(f, "Failed to read username from stdin."),
            Self::PasswordRead(..) => write!(f, "Failed to read password from stdin."),
            Self::PlainTextRead(..) => write!(f, "Failed to read value from stdin."),
            Self::SecretRead(..) => write!(f, "Failed to read secret value from stdin."),

            #[cfg(feature = "tokio")]
            Self::StdinReadJoin(_) => write!(f, "Failed to wait for stdin task to complete."),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PromptWrite { error, .. } => Some(error),
            Self::StdErrFlush(error) => Some(error),
            Self::UsernameRead(error) => Some(error),
            Self::PasswordRead(error) => Some(error),
            Self::PlainTextRead(error) => Some(error),
            Self::SecretRead(error) => Some(error),

            #[cfg(feature = "tokio")]
            Self::StdinReadJoin(error) => Some(error),
        }
    }
}
