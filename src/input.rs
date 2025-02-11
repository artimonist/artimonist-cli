use super::unicode::UnicodeUtils;
use artimonist::{Matrix, ToMatrix};
use inquire::validator::Validation;
use inquire::{Confirm, InquireError, PasswordDisplayMode};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::path::Path;

pub trait Formatter<T> {
    fn format(self) -> Option<T>;
}
impl Formatter<char> for &str {
    fn format(self) -> Option<char> {
        self.unicode_decode().chars().next()
    }
}
impl Formatter<String> for &str {
    fn format(self) -> Option<String> {
        Some(self.unicode_decode().chars().take(20).collect())
    }
}

pub struct Input;

impl Input {
    /// Input diagram from file
    pub fn diagram_file<T: Debug>(path: &str) -> Result<Matrix<7, 7, T>, IoError>
    where
        for<'a> &'a str: Formatter<T>,
    {
        let file = File::open(Path::new(path))?;
        let buffered = BufReader::new(file);
        let mvs = buffered
            .lines()
            .take(7)
            .map(|r| match r {
                Ok(ln) => ln
                    .split_whitespace()
                    .take(7)
                    .map(|s| s.trim_matches('\"').format())
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<_>>();
        Ok(mvs.to_matrix())
    }

    /// Input diagram
    pub fn matrix<T: Debug>() -> Result<Matrix<7, 7, T>, InquireError>
    where
        for<'a> &'a str: Formatter<T>,
    {
        let mut mvs: Vec<_> = vec![];
        for i in 1..=7 {
            let vs: Vec<_> = inquire::Text::new(&format!("row ({i})"))
                .with_initial_value(&"\"\"  ".repeat(7))
                .with_help_message("Fill characters in quotes.")
                .prompt()?
                .split_whitespace()
                .map(|s| s.trim_matches('\"').format())
                .collect();
            mvs.push(vs);
        }
        Ok(mvs.to_matrix())
    }

    // Input password as salt
    pub fn password(salt: bool) -> Result<String, InquireError> {
        let validator = |v: &str| match v.chars().count() {
            ..5 => Ok(Validation::Invalid(
                "Encryption key must have at least 5 characters.".into(),
            )),
            _ => Ok(Validation::Valid),
        };
        inquire::Password::new("Encryption Key: ")
            .with_display_mode(PasswordDisplayMode::Masked)
            .with_display_toggle_enabled()
            .with_custom_confirmation_message("Encryption Key (confirm):")
            .with_custom_confirmation_error_message("The keys don't match.")
            .with_validator(validator)
            .with_formatter(&|_| "Input received".into())
            .with_help_message(match salt {
                true => "Program use encryption key as salt.",
                false => "Input encryption key",
            })
            .prompt()
    }

    pub fn confirm_overwrite(msg: &str) -> Result<bool, InquireError> {
        if !msg.is_empty() {
            println!("{msg}");
        }
        Confirm::new("Confirm overwrite file?")
            .with_default(false)
            .with_help_message("This operation will overwrite file.")
            .prompt()
    }
}
