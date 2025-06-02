pub trait CheckInputKey {
    fn is_private(&self) -> bool;
    fn is_encrypted(&self) -> bool;
    fn is_master(&self) -> bool;
    fn is_mnemonic(&self) -> bool;
}

pub trait ConfirmOverwrite {
    fn confirm_overwrite(&self) -> bool;
}

pub trait InquirePassword {
    fn inquire_password(&mut self, as_salt: bool) -> anyhow::Result<()>;
}

impl CheckInputKey for str {
    #[inline]
    fn is_private(&self) -> bool {
        self.starts_with(['K', 'L', '5']) && self.len() == 52
    }

    #[inline]
    fn is_encrypted(&self) -> bool {
        self.starts_with("6P") && self.len() == 58
    }

    #[inline]
    fn is_master(&self) -> bool {
        self.starts_with("xprv") && self.len() == 111
    }

    #[inline]
    fn is_mnemonic(&self) -> bool {
        matches!(self.split_whitespace().count(), 12 | 15 | 18 | 21 | 24)
    }
}

impl ConfirmOverwrite for Option<String> {
    fn confirm_overwrite(&self) -> bool {
        if crate::TESTING_MODE {
            return true; // Skip confirmation in testing mode
        }

        if let Some(path) = self {
            if std::path::Path::new(path).exists() {
                println!("File exists.");
                return inquire::Confirm::new("Confirm overwrite file?")
                    .with_default(false)
                    .with_help_message("This operation will overwrite file.")
                    .prompt()
                    .unwrap_or(false);
            }
        }
        true
    }
}

impl InquirePassword for String {
    fn inquire_password(&mut self, as_salt: bool) -> anyhow::Result<()> {
        use super::unicode::UnicodeUtils;
        use inquire::validator::Validation;

        if crate::TESTING_MODE {
            return Ok(()); // Skip if already set
        }

        const INVALID_MSG: &str = "Encryption key must have at least 5 characters.";
        let validator = |v: &str| {
            if v.unicode_decode().chars().count() < 5 {
                Ok(Validation::Invalid(INVALID_MSG.into()))
            } else {
                Ok(Validation::Valid)
            }
        };

        *self = inquire::Password::new("Encryption Key: ")
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .with_display_toggle_enabled()
            .with_custom_confirmation_message("Encryption Key (confirm):")
            .with_custom_confirmation_error_message("The keys don't match.")
            .with_validator(validator)
            .with_formatter(&|_| "Input received".into())
            .with_help_message(if as_salt {
                "Program use encryption key as salt."
            } else {
                "Input encryption key"
            })
            .prompt()?
            .unicode_decode();
        Ok(())
    }
}
