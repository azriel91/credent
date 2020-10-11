use std::{fmt, fmt::Write};

use crossterm::style::{Attribute, Attributes, Color, ContentStyle};

#[derive(Debug)]
pub struct Prompt;

impl Prompt {
    pub fn username() -> String {
        format!("{} ", Colours::prompt_label().apply("Username:"))
    }

    pub fn password() -> String {
        format!(
            "{password} {hint}{colon} ",
            password = Colours::prompt_label().apply("Password"),
            hint = Colours::prompt_hint().apply("(input is hidden)"),
            colon = Colours::prompt_label().apply(":")
        )
    }
}

/// Colours for UI output on terminal
#[derive(Debug)]
pub struct Colours;

impl Colours {
    /// Logo left colour.
    pub fn logo_left() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Blue),
            background_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }
    }

    /// Logo left colour.
    pub fn logo_right() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Grey),
            background_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }
    }

    /// Informative label colour.
    pub fn informative_label() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Yellow),
            background_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }
    }

    /// Informative value colour.
    pub fn informative_value() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::White),
            background_color: None,
            attributes: Attributes::from(Attribute::NormalIntensity),
        }
    }

    /// Input label colour.
    pub fn prompt_label() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Green),
            background_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }
    }

    /// Input hint colour.
    pub fn prompt_hint() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::DarkGreen),
            background_color: None,
            attributes: Attributes::from(Attribute::NormalIntensity),
        }
    }

    /// Output label colour.
    pub fn output_label() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Blue),
            background_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }
    }

    /// Output hint colour.
    pub fn output_hint() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::DarkBlue),
            background_color: None,
            attributes: Attributes::from(Attribute::NormalIntensity),
        }
    }

    /// Error label colour.
    #[allow(dead_code)]
    pub fn error_label() -> ContentStyle {
        ContentStyle {
            foreground_color: Some(Color::Red),
            background_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }
    }
}

/// Text representation of the Credent Logo.
#[derive(Debug)]
pub struct Logo;

impl Logo {
    /// Recommended size for the ASCII logo buffer.
    pub const ASCII_BUFFER_SIZE: usize = 192;

    /// Returns a String with the Credent ASCII logo.
    ///
    /// See the [`Logo::ascii_plain_write`] function if you wish to provide
    /// your own buffer.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn ascii_plain_write<W>(mut buffer: W)
    where
        W: Write,
    {
        Self::write_logo(&mut buffer, false);
    }

    /// Returns a String with the Credent ASCII logo with ANSI colours.
    ///
    /// See the [`Logo::ascii_coloured_write`] function if you wish to
    /// provide your own buffer.
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
    #[allow(dead_code)]
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
                    let left = Colours::logo_left().apply(left);
                    let right = Colours::logo_right().apply(right);

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
