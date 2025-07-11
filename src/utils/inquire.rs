pub fn inquire_password(as_salt: bool) -> anyhow::Result<String> {
    use super::unicode::UnicodeUtils;
    use inquire::validator::Validation;

    const INVALID_MSG: &str = "Encryption key must have at least 5 characters.";
    let validator = |v: &str| {
        if v.unicode_decode().chars().count() < 5 {
            Ok(Validation::Invalid(INVALID_MSG.into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let pwd = inquire::Password::new("Encryption Key: ")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .with_display_toggle_enabled()
        .with_custom_confirmation_message("Encryption Key (confirm):")
        .with_custom_confirmation_error_message("The keys don't match.")
        .with_validator(validator)
        .with_formatter(&|_| "Input received".into())
        .with_help_message(if as_salt {
            "Program use encryption key as salt. (Toggle display by CTRL+R)"
        } else {
            "Input encryption key. (Toggle display by CTRL+R)"
        })
        .prompt()?
        .unicode_decode();
    Ok(pwd)
}

use artimonist::Language;
/// Prompt user to choose a mnemonic language.
pub fn select_language(langs: &[Language]) -> anyhow::Result<Language> {
    use inquire::Select;

    let options = langs.iter().map(|&v| format!("{v:?}")).collect();
    let choice = Select::new("Which mnemonic language do you want?", options)
        .with_page_size(langs.len())
        .prompt()?;
    Ok(choice.parse()?)
}
