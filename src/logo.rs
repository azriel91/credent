use std::{fmt, fmt::Write};

use crate::Colours;

/// Text representation of the Credent Logo.
#[derive(Debug)]
pub struct Logo;

impl Logo {
    /// Recommended size for the ASCII logo buffer.
    pub const ASCII_BUFFER_SIZE: usize = 192;

    /// Returns a String with the Credent ASCII logo.
    ///
    /// See the [`Logo::ascii_plain_write`] function if you wish to provide your
    /// own buffer.
    pub fn ascii_plain() -> String {
        let mut buffer = String::with_capacity(Self::ASCII_BUFFER_SIZE);
        Self::ascii_plain_write(&mut buffer);

        buffer
    }

    /// Writes the ascii logo to the given buffer.
    ///
    /// The buffer is recommended to have [`ASCII_BUFFER_SIZE`] free bytes.
    ///
    /// # Parameters
    ///
    /// * `buffer`: Buffer to write to.
    pub fn ascii_plain_write<W>(mut buffer: W)
    where
        W: Write,
    {
        Self::write_logo(&mut buffer, false);
    }

    /// Returns a String with the Credent ASCII logo with ANSI colours.
    ///
    /// See the [`Logo::ascii_coloured_write`] function if you wish to provide
    /// your own buffer.
    pub fn ascii_coloured() -> String {
        let mut buffer = String::with_capacity(Self::ASCII_BUFFER_SIZE);
        Self::write_logo(&mut buffer, true);

        buffer
    }

    /// Writes the ascii logo to the given buffer with ANSI colours.
    ///
    /// The buffer is recommended to have [`ASCII_BUFFER_SIZE`] free bytes.
    ///
    /// # Parameters
    ///
    /// * `buffer`: Buffer to write to.
    pub fn ascii_coloured_write<W>(mut buffer: W)
    where
        W: Write,
    {
        Self::write_logo(&mut buffer, true);
    }

    fn write_logo<W>(buffer: W, coloured: bool)
    where
        W: Write,
    {
        let logo_left = [
            r#"            "#,
            r#" ___ ___ ___"#,
            r#"|  _|  _| -_"#,
            r#"|___|_| |___"#,
        ];
        let logo_right = [
            r#"   _         _"#,
            r#" _| |___ ___| |_"#,
            r#"| . | -_|   |  _|"#,
            r#"|___|___|_|_|_|"#,
        ];
        logo_left
            .iter()
            .zip(logo_right.iter())
            .try_fold(buffer, |mut buffer, (left, right)| {
                if coloured {
                    let left = Colours::LOGO_LEFT.clone().apply(left);
                    let right = Colours::LOGO_RIGHT.clone().apply(right);

                    write!(&mut buffer, "{}", left)?;
                    writeln!(&mut buffer, "{}", right)?;
                } else {
                    write!(&mut buffer, "{}", left)?;
                    writeln!(&mut buffer, "{}", right)?;
                }

                Result::<W, fmt::Error>::Ok(buffer)
            })
            .expect("Failed to write logo.");
    }
}
