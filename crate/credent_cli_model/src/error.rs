use std::fmt;

#[cfg(feature = "smol")]
type IoError = smol::io::Error;

#[cfg(not(feature = "smol"))]
type IoError = std::io::Error;

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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PromptWrite { prompt, .. } => {
                write!(f, "Failed to write prompt to stderr. Prompt: `{}`", prompt)
            }
            Self::StdErrFlush(..) => write!(f, "Failed to flush `stderr`."),
            Self::UsernameRead(..) => write!(f, "Failed to read username."),
            Self::PasswordRead(..) => write!(f, "Failed to read password."),
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
        }
    }
}
