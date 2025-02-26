use artimonist::Language;
use inquire::validator::Validation;
use inquire::{Confirm, InquireError, PasswordDisplayMode, Select};

pub struct Input;

impl Input {
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

use artimonist::Language::*;
const LANGUAGES: [Language; 10] = [
    English,
    Japanese,
    Korean,
    Spanish,
    SimplifiedChinese,
    TraditionalChinese,
    French,
    Italian,
    Czech,
    Portuguese,
];

fn language(name: &str) -> Option<Language> {
    match &name.to_lowercase()[..] {
        "english" => Some(English),
        "japanese" => Some(Japanese),
        "korean" => Some(Korean),
        "spanish" => Some(Spanish),
        "simplifiedchinese" => Some(SimplifiedChinese),
        "traditionalchinese" => Some(TraditionalChinese),
        "french" => Some(French),
        "italian" => Some(Italian),
        "czech" => Some(Czech),
        "portuguese" => Some(Portuguese),
        _ => None,
    }
}
