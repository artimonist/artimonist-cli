use artimonist::Language;
use artimonist::Language::*;

pub trait ChooseLanguage {
    /// Prompt user to choose a mnemonic language.
    fn choose_language(&mut self) -> anyhow::Result<()>;
}

impl ChooseLanguage for Language {
    #[cfg(feature = "automatic")]
    fn choose_language(&mut self) -> anyhow::Result<()> {
        *self = English;
        Ok(())
    }

    #[cfg(not(feature = "automatic"))]
    fn choose_language(&mut self) -> anyhow::Result<()> {
        let options = LANGUAGES.map(|v| format!("{v:?}")).to_vec();
        let choice = inquire::Select::new("Which mnemonic language do you want?", options)
            .with_page_size(LANGUAGES.len())
            .prompt()?;
        let wrap: LanguageWrap = choice.into();
        *self = wrap.0;
        Ok(())
    }
}

#[cfg(not(feature = "automatic"))]
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

#[cfg(not(feature = "automatic"))]
struct LanguageWrap(pub Language);

#[cfg(not(feature = "automatic"))]
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
