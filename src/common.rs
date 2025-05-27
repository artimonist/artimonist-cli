pub trait CheckInputKey {
    fn is_private(&self) -> bool;
    fn is_encrypted(&self) -> bool;
    fn is_master(&self) -> bool;
    fn is_mnemonic(&self) -> bool;
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

pub trait ConfirmOverwrite {
    fn confirm_overwrite(&self) -> bool;
}

impl ConfirmOverwrite for Option<String> {
    fn confirm_overwrite(&self) -> bool {
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
