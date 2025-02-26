use super::unicode::UnicodeUtils;
use artimonist::{Language, Matrix, ToMatrix};
use inquire::validator::Validation;
use inquire::{Confirm, InquireError, PasswordDisplayMode, Select};
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
    pub fn diagram_file<T: Debug>(path: &str) -> Result<Matrix<T, 7, 7>, IoError>
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
    pub fn matrix<T: Debug>() -> Result<Matrix<T, 7, 7>, InquireError>
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
    pub fn password(as_salt: bool) -> Result<String, InquireError> {
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
            .with_help_message(match as_salt {
                true => "Program use encryption key as salt.",
                false => "Input encryption key",
            })
            .prompt()
    }

    pub fn choice_language() -> Result<Language, InquireError> {
        let options = LANGUAGES.map(|v| format!("{v:?}")).to_vec();
        let choice = Select::new("Which mnemonic language do you want?", options)
            .with_page_size(LANGUAGES.len())
            .prompt()?;
        Ok(language(&choice).unwrap())
    }

    pub fn confirm_overwrite(msg: &str) -> Result<bool, InquireError> {
        if !msg.is_empty() {
            println!("File exists.");
        }
        Confirm::new("Confirm overwrite file?")
            .with_default(false)
            .with_help_message("This operation will overwrite file.")
            .prompt()
    }
}

const LANGUAGES: [Language; 10] = [
    Language::English,
    Language::Japanese,
    Language::Korean,
    Language::Spanish,
    Language::SimplifiedChinese,
    Language::TraditionalChinese,
    Language::French,
    Language::Italian,
    Language::Czech,
    Language::Portuguese,
];

fn language(name: &str) -> Option<Language> {
    match &name.to_lowercase()[..] {
        "english" => Some(Language::English),
        "japanese" => Some(Language::Japanese),
        "korean" => Some(Language::Korean),
        "spanish" => Some(Language::Spanish),
        "simplifiedchinese" => Some(Language::SimplifiedChinese),
        "traditionalchinese" => Some(Language::TraditionalChinese),
        "french" => Some(Language::French),
        "italian" => Some(Language::Italian),
        "czech" => Some(Language::Czech),
        "portuguese" => Some(Language::Portuguese),
        _ => None,
    }
}
