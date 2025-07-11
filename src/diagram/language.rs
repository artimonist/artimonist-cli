use artimonist::Language;
use artimonist::Language::*;

pub trait ChooseLanguage {
    /// Prompt user to choose a mnemonic language.
    fn choose_language(&mut self) -> anyhow::Result<()>;
}

impl ChooseLanguage for Language {
    fn choose_language(&mut self) -> anyhow::Result<()> {
        if crate::AUTOMATIC_MODE {
            *self = English; // Skip prompt in automatic mode
            return Ok(());
        }

        let options = Language::all().map(|v| format!("{v:?}")).to_vec();
        let choice = inquire::Select::new("Which mnemonic language do you want?", options)
            .with_page_size(Language::all().len())
            .prompt()?;
        *self = choice.parse()?;
        Ok(())
    }
}
