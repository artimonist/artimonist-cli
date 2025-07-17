use artimonist::bitcoin::bip32::DerivationPath;
use artimonist::{Mnemonic, Xpriv, Xpub};

#[derive(clap::Parser, Debug)]
pub struct Bip32Command {
    /// Master key or Mnemonic phrase
    #[clap(name = "MNEMONIC|MASTER_KEY")]
    pub key: MasterKey,

    /// Derivation path
    pub path: Option<DerivationPath>,

    /// Password as salt
    #[clap(hide = true, long)]
    pub password: Option<String>,
}

/// Master key or Mnemonic string
#[derive(Debug, Clone)]
pub enum MasterKey {
    /// Mnemonic phrase
    Mnemonic(Mnemonic),
    /// Extended private key
    Xpriv(Xpriv),
    /// Extended public key
    Xpub(Xpub),
}

impl std::str::FromStr for MasterKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("xprv") {
            Ok(MasterKey::Xpriv(Xpriv::from_str(s)?))
        } else if s.starts_with("xpub") {
            Ok(MasterKey::Xpub(Xpub::from_str(s)?))
        } else {
            Ok(MasterKey::Mnemonic(Mnemonic::from_str(s)?))
        }
    }
}

impl Bip32Command {
    pub fn is_mnemonic(&self) -> bool {
        matches!(self.key, MasterKey::Mnemonic(_))
    }
    pub fn is_xpub(&self) -> bool {
        matches!(self.key, MasterKey::Xpub(_))
    }
}

pub fn inquire_derivation_path() -> anyhow::Result<DerivationPath> {
    use inquire::Text;
    use std::str::FromStr;

    let path = Text::new("Enter derivation path")
        .with_initial_value("m/0'/0'")
        .with_help_message("e.g. m/44'/0'/0'/0/0")
        .prompt()?;

    Ok(DerivationPath::from_str(&path)?)
}
