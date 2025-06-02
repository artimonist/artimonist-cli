use artimonist::Language;
use artimonist::Language::*;

pub trait ChooseLanguage {
    /// Prompt user to choose a mnemonic language.
    fn choose_language(&mut self) -> anyhow::Result<()>;
}

impl ChooseLanguage for Language {
    fn choose_language(&mut self) -> anyhow::Result<()> {
        if crate::TESTING_MODE {
            *self = English; // Skip prompt in testing mode
            return Ok(());
        }

        let options = LANGUAGES.map(|v| format!("{v:?}")).to_vec();
        let choice = inquire::Select::new("Which mnemonic language do you want?", options)
            .with_page_size(LANGUAGES.len())
            .prompt()?;
        let wrap: LanguageWrap = choice.into();
        *self = wrap.0;
        Ok(())
    }
}

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

struct LanguageWrap(pub Language);

impl From<String> for LanguageWrap {
    fn from(value: String) -> Self {
        let lang = match value.to_lowercase().as_str() {
            "english" => English,
            "japanese" => Japanese,
            "korean" => Korean,
            "spanish" => Spanish,
            "simplifiedchinese" => SimplifiedChinese,
            "traditionalchinese" => TraditionalChinese,
            "french" => French,
            "italian" => Italian,
            "czech" => Czech,
            "portuguese" => Portuguese,
            _ => English,
        };
        LanguageWrap(lang)
    }
}
